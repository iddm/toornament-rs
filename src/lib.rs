//! Client library for the [Toornament](https://www.toornament.com) web API.
//!
//! Log in to Toornament with `Toornament::with_application`.
//! Call API methods to interact with the service.
//!
//! For Toornament API documentation [look here]
//! (https://developer.toornament.com/overview/get-started).
//!
//! For examples, see the `examples` directory in the source tree.
#![warn(missing_docs)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate hyper;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate chrono;

use reqwest::header::{ Bearer, Authorization, ContentType };
use std::collections::HashMap;
use std::io::Read;

#[macro_use]
mod macroses;
mod matches;
mod error;
mod tournaments;
mod disciplines;
mod games;
mod filters;
mod participants;
mod common;

pub use error::{ Result, Error };
pub use matches::{ Match, MatchId, Matches, MatchType, MatchResult };
pub use disciplines::{ DisciplineId, Discipline, Disciplines };
pub use filters::{ MatchFilter };
pub use tournaments::{
    ParticipantType,
    TournamentId,
    Tournament,
    Tournaments,
    TournamentStatus,
};


const API_BASE: &'static str = "https://api.toornament.com";

header! {
    /// X-Api-Key header is used in toornament authentication mechanism.
    /// It must point to a valid application key (String).
    (XApiKey, "X-Api-Key") => [String]
}

#[derive(Debug, Clone, Copy, Ord, Eq, PartialOrd, PartialEq, Hash)]
enum Endpoint {
    OauthToken,
    Disciplines,
    PublicTournaments,
    MyTournaments,
    Matches,
}

lazy_static! {
    static ref API_EP: HashMap<Endpoint, &'static str> = {
        let mut m: HashMap<Endpoint, &'static str> = HashMap::new();
        m.insert(Endpoint::OauthToken, "/oauth/v2/token");
        m.insert(Endpoint::Disciplines, "/v1/disciplines");
        m.insert(Endpoint::PublicTournaments, "/v1/tournaments");
        m.insert(Endpoint::MyTournaments, "/v1/me/tournaments");
        /// Matches endpoint requires substitution of :tournament_id:
        m.insert(Endpoint::Matches, "/v1/tournaments/:tournament_id:/matches");
        m
    };
}

fn check_status(response: reqwest::Result<reqwest::Response>)
    -> Result<reqwest::Response> {
    let response = response?;
    if !response.status().is_success() {
        return Err(Error::from_response(response))
    }
    Ok(response)
}

fn retry<F: Fn() -> reqwest::RequestBuilder>(f: F)
    -> Result<reqwest::Response> {
    let f2 = || check_status(f().send());
    // retry on a ConnectionAborted, which occurs if it's been a while since the last request
    match f2() {
        Err(_) => f2(),
        other => other
    }
}

fn parse_token<R: Read>(json_str: R) -> Result<String> {
    #[derive(Deserialize)]
    struct AccessToken {
        access_token: String,
    }
    let json: AccessToken = serde_json::from_reader(json_str)?;
    Ok(json.access_token)
}

fn get_ep_address(ep: Endpoint) -> Result<String> {
    if API_EP.contains_key(&ep) {
        return Ok(format!("{}{}", API_BASE, API_EP.get(&ep).unwrap())) // safe
    }
    Err(Error::Other("Attempted to use unexistent endpoint"))
}

fn match_filter_to_get_string(f: Option<MatchFilter>) -> String {
    let f = match f {
        Some(f) => f,
        None => return String::default(),
    };
    let mut out = String::new();
    match f.featured {
        Some(f) => out.push_str(&format!("&featured={}", if f { 1 } else { 0 })),
        None => {},
    }
    match f.has_result {
        Some(r) => out.push_str(&format!("&has_result={}", if r { 1 } else { 0 })),
        None => {},
    }
    match f.sort {
        Some(s) => out.push_str(&format!("&sort={}", s.to_string())),
        None => {},
    }
    match f.participant_id {
        Some(i) => out.push_str(&format!("&participant_id={}", i.0)),
        None => {},
    }
    match f.tournament_ids {
        Some(ref i) => out.push_str(&format!("&tournament_ids={}",
                                             i.iter()
                                              .map(|i| i.0.as_str())
                                              .collect::<Vec<&str>>()
                                              .join(","))),
        None => {},
    }
    out.push_str(&format!("&with_games={}", if f.with_games { 1 } else { 0 }));
    match f.before_date {
        Some(d) => out.push_str(&format!("&before_date={}", d)),
        None => {},
    }
    match f.after_date {
        Some(d) => out.push_str(&format!("&after_date={}", d)),
        None => {},
    }
    match f.page {
        Some(p) => out.push_str(&format!("&page={}", p)),
        None => {},
    }
    out
}

/// Main structure. Should be your point of start using the service.
/// This struct covers all the `toornament` API.
#[derive(Debug)]
pub struct Toornament {
    client: reqwest::Client,
    keys: (String, String, String),
    oauth_token: String,
}
impl Toornament {
    /// Creates new `Toornament` object with client credentials
    /// which is your user API_Token, application's client id and secret.
    /// You may obtain application's credentials [here]
    /// (https://developer.toornament.com/applications/) (You must be logged in to open the page).
    /// This method connects to the toornament service and if there is a error it returns the `Error`
    /// object and on success it returns `Toornament` object.
    pub fn with_application<S: Into<String>>(api_token: S,
                                             client_id: S,
                                             client_secret: S) -> Result<Toornament> {

        let client = reqwest::Client::new().unwrap(); // intented to crash
        let keys = (api_token.into(), client_id.into(), client_secret.into());
        let body = format!("grant_type=client_credentials&client_id={}&client_secret={}",
                            keys.1,
                            keys.2);
        let address = get_ep_address(Endpoint::OauthToken)?;
        let response = retry(|| client.post(&address)
                                      .header(ContentType::form_url_encoded())
                                      .body(body.as_str()))?;
        let got_token = parse_token(response)?;
        Ok(Toornament {
            client: client,
            keys: keys,
            oauth_token: got_token,
        })
    }

    /// [Returns either a collection of disciplines]
    /// (https://developer.toornament.com/doc/disciplines#get:disciplines) if id is None or
    /// [a disciplines with the detail of his features]
    /// (https://developer.toornament.com/doc/disciplines#get:disciplines:id)
    pub fn disciplines(&self, id: Option<DisciplineId>) -> Result<Disciplines> {
        let address;
        let id_is_set = id.is_some();
        if let Some(id) = id {
            debug!("Getting disciplines with id: {:?}", id);
            address = format!("{}/{}",
                              get_ep_address(Endpoint::Disciplines)?,
                              id.0);
        } else {
            debug!("Getting all disciplines");
            address = get_ep_address(Endpoint::Disciplines)?;
        }
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;
        if id_is_set {
            Ok(Disciplines(vec![serde_json::from_reader::<_, Discipline>(response)?]))
        } else {
            Ok(serde_json::from_reader(response)?)
        }
    }

    /// [Returns a collection of public tournaments filtered and sorted by the given query
    /// parameters. A maximum of 20 tournaments will be returned. Only public tournaments are
    /// visible.](https://developer.toornament.com/doc/tournaments#get:tournaments) if id is None or
    /// [a detailed information about one tournament. The tournament must be public.]
    /// (https://developer.toornament.com/doc/tournaments#get:tournaments:id)
    pub fn tournaments(&self,
                       id: Option<TournamentId>,
                       with_streams: bool) -> Result<Tournaments> {
        let address;
        let id_is_set = id.is_some();
        if let Some(id) = id {
            debug!("Getting tournament with id: {:?}", id);
            address = format!("{}/{}?with_streams={}",
                              get_ep_address(Endpoint::PublicTournaments)?,
                              id.0,
                              if with_streams { "1" } else { "0" });
        } else {
            debug!("Getting all tournaments");
            address = format!("{}?with_streams={}",
                              get_ep_address(Endpoint::PublicTournaments)?,
                              if with_streams { "1" } else { "0" });
        }
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;
        if id_is_set {
            Ok(Tournaments(vec![serde_json::from_reader::<_, Tournament>(response)?]))
        } else {
            Ok(serde_json::from_reader(response)?)
        }
    }

    /// [Updates some of the editable information on a tournament.]
    /// (https://developer.toornament.com/doc/tournaments#patch:tournaments:id) if `tournament.id`
    /// is set otherwise [creates a tournament]
    /// (https://developer.toornament.com/doc/tournaments#post:tournaments).
    pub fn edit_tournament(&self,
                           tournament: Tournament) -> Result<Tournament> {
        debug!("Editing tournament: {:#?}", tournament);
        let address;
        let id_is_set = tournament.id.is_some();
        if let Some(id) = tournament.id.clone() {
            address = format!("{}/{}",
                              get_ep_address(Endpoint::PublicTournaments)?,
                              id.0);
        } else {
            address = format!("{}", get_ep_address(Endpoint::PublicTournaments)?);
        }
        let body = serde_json::to_string(&tournament)?;
        let response;
        if id_is_set {


            response = retry(|| self.client.patch(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        } else {
            response = retry(|| self.client.post(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;
        }
        Ok(serde_json::from_reader(response)?)
    }

    /// [Deletes a tournament, its participants and all its matches]
    /// (https://developer.toornament.com/doc/tournaments#delete:tournaments:id).
    pub fn delete_tournament(&self,
                             id: TournamentId) -> Result<()> {
        debug!("Deleting tournament by id: {:?}", id);
        let address = format!("{}/{}",
                              get_ep_address(Endpoint::PublicTournaments)?,
                              id.0);
        let _ = retry(|| self.client.delete(&address)
                                    .header(XApiKey(self.keys.0.clone()))
                                    .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;
        Ok(())
    }

    /// [Returns the private and public tournaments on which the authenticated user has access.
    /// The result is filtered, sorted and paginated by the given query parameters. A maximum of
    /// 50 tournaments is returned (per page).]
    /// (https://developer.toornament.com/doc/tournaments#get:metournaments)
    pub fn my_tournaments(&self) -> Result<Tournaments> {
        debug!("Getting all tournaments");
        let address = get_ep_address(Endpoint::MyTournaments)?;
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;
        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of matches from one tournament. The collection may be filtered and
    /// sorted by optional query parameters. The tournament must be public to have access to its
    /// matches, meaning the tournament organizer has published it.]
    /// (https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches)
    pub fn matches(&self, id: TournamentId, match_id: Option<MatchId>, with_games: bool) -> Result<Matches> {
        let response = match match_id {
            Some(match_id) => {
                debug!("Getting matches by tournament id and match id: {:?} / {:?}", id, match_id);
                let ep = format!("{}/{}?with_games={}",
                                 get_ep_address(Endpoint::Matches)?,
                                 match_id.0,
                                 if with_games { "1" } else { "0" });
                let address = ep.replace(":tournament_id:", &id.0);
                retry(|| self.client.get(&address)
                                    .header(XApiKey(self.keys.0.clone()))
                                    .header(Authorization(Bearer { token: self.oauth_token.clone() })))?
            },
            None => {
                debug!("Getting matches by tournament id: {:?}", id);
                let ep = format!("{}?with_games={}",
                                 get_ep_address(Endpoint::Matches)?,
                                 if with_games { "1" } else { "0" });
                let address = ep.replace(":tournament_id:", &id.0);
                retry(|| self.client.get(&address)
                                    .header(XApiKey(self.keys.0.clone())))?
            },
        };

        Ok(serde_json::from_reader(response)?)
    }
    
    /// [Retrieve a collection of matches from a specific discipline, filtered and sorted by the
    /// given query parameters. It might be a list of matches from different tournaments, but only
    /// from public tournaments. The matches are returned by 20.]
    /// (https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches)
    pub fn matches_by_discipline(&self, id: DisciplineId, filter: Option<MatchFilter>)
        -> Result<Matches> {
        debug!("Getting matches by tournament id: {:?}", id);
        let address = format!("{}/{}/matches?{}",
                              get_ep_address(Endpoint::Matches)?,
                              id.0,
                              match_filter_to_get_string(filter));
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [If you need to make changes on your match data, you are able to do so by patching one or
    /// several fields of your match.](https://developer.toornament.com/doc/matches#patch:tournaments:tournament_id:matches:id)
    pub fn update_match(&self,
                        id: TournamentId,
                        match_id: MatchId,
                        updated_match: Match) -> Result<Match> {
        debug!("Updating a match result by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let address = format!("{}/{}/matches/{}",
                              get_ep_address(Endpoint::Matches)?,
                              id.0,
                              match_id.0);
        let body = serde_json::to_string(&updated_match)?;
        let response = retry(|| self.client.patch(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed result about one match.]
    /// (https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches:id:result)
    pub fn match_result(&self, id: TournamentId, match_id: MatchId) -> Result<MatchResult> {
        debug!("Getting match result by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let address = format!("{}/{}/matches/{}/result",
                              get_ep_address(Endpoint::Matches)?,
                              id.0,
                              match_id.0);
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update or create detailed result about one match.]
    /// (https://developer.toornament.com/doc/matches#put:tournaments:tournament_id:matches:id:result)
    pub fn set_match_result(&self,
                            id: TournamentId,
                            match_id: MatchId,
                            result: MatchResult) -> Result<MatchResult> {
        debug!("Setting match result by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let address = format!("{}/{}/matches/{}/result",
                              get_ep_address(Endpoint::Matches)?,
                              id.0,
                              match_id.0);
        let body = serde_json::to_string(&result)?;
        let response = retry(|| self.client.put(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }
}
