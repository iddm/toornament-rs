//! Client library for the [Toornament](https://www.toornament.com) web API.
//!
//! Log in to Toornament with `Toornament::with_application`.
//! Call API methods to interact with the service.
//!
//! For Toornament API documentation [look here]
//! (https://developer.toornament.com/overview/get-started).
//!
//! For examples, see the `examples` directory in the source tree.
//!
//! # Usage
//!
//! Start by creating and instance `Toornament` structure and then perform requests:
//!
//! ```rust,no_run
//! use toornament::*;
//!
//! let toornament = Toornament::with_application("API_TOKEN",
//!                                               "CLIENT_ID",
//!                                               "CLIENT_SECRET").unwrap();
//! println!("Disciplines: {:?}\n", toornament.disciplines(None));
//! ```
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
mod permissions;
mod stages;
mod videos;
mod opponents;
mod streams;
mod common;

pub use error::{
    Result,
    Error,
    ToornamentServiceError,
    ToornamentErrorType,
    ToornamentErrorScope,
    ToornamentError,
    ToornamentErrors,
};
pub use common::{ TeamSize, MatchResultSimple, Date };
pub use matches::{
    Match,
    MatchId,
    Matches,
    MatchType,
    MatchResult,
    MatchStatus,
    MatchFormat,
};
pub use games::{ GameNumber, Game, Games };
pub use stages::{ StageNumber, StageType, Stage, Stages };
pub use videos::{ VideoCategory, Video, Videos };
pub use opponents::{ Opponent, Opponents, };
pub use permissions::{
    PermissionId,
    PermissionAttribute,
    PermissionAttributes,
    Permission,
    Permissions,
};
pub use participants::{
    ParticipantId,
    ParticipantType,
    ParticipantLogo,
    Participant,
    Participants,
    CustomFieldType,
    CustomField,
    CustomFields,
};
pub use disciplines::{ DisciplineId, Discipline, Disciplines, AdditionalFields };
pub use filters::{
    CreateDateSortFilter,
    DateSortFilter,
    MatchFilter,
    TournamentParticipantsFilter,
    TournamentVideosFilter,
};
pub use tournaments::{
    TournamentId,
    Tournament,
    Tournaments,
    TournamentStatus,
};
pub use streams:: { StreamId, Stream, Streams };

/// Macro only for internal use with the `Toornament` object (relies on it's fields)
macro_rules! request {
    ($toornament:ident, $method:ident, $address:expr) => {
        retry(|| $toornament.client.$method($address)
                                   .header(XApiKey($toornament.keys.0.clone()))
                                   .header(Authorization(Bearer {
                                       token: $toornament.oauth_token.clone()
                                   })))
    }
}

/// Macro only for internal use with the `Toornament` object (relies on it's fields)
macro_rules! request_body {
    ($toornament:ident, $method:ident, $address:expr, $body:expr) => {
        retry(|| $toornament.client.$method($address)
                                   .body($body)
                                   .header(XApiKey($toornament.keys.0.clone()))
                                   .header(Authorization(Bearer {
                                       token: $toornament.oauth_token.clone()
                                   })))
    };
}


const API_BASE: &'static str = "https://api.toornament.com";

mod custom_headers {
    header! {
        /// X-Api-Key header is used in toornament authentication mechanism.
        /// It must point to a valid application key (String).
        (XApiKey, "X-Api-Key") => [String]
    }
}
use custom_headers::*;

#[derive(Debug, Clone, Copy, Ord, Eq, PartialOrd, PartialEq, Hash)]
enum Endpoint {
    OauthToken,
    Disciplines,
    PublicTournaments,
    MyTournaments,
    Matches,
    Participants,
    Permissions,
    Stages,
    Videos,
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
        m.insert(Endpoint::Participants, "/v1/tournaments/:tournament_id:/participants");
        m.insert(Endpoint::Permissions, "/v1/tournaments/:tournament_id:/permissions");
        m.insert(Endpoint::Stages, "/v1/tournaments/:tournament_id:/stages");
        m.insert(Endpoint::Videos, "/v1/tournaments/:tournament_id:/videos");
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
    #[derive(Debug, Deserialize)]
    struct AccessToken {
        access_token: String,
        expires_in: u64,
        token_type: String,
        scope: Option<String>,
    }
    let json: AccessToken = serde_json::from_reader(json_str)?;
    debug!("Toornament access token information: {:?}", json);
    Ok(json.access_token)
}

fn get_ep_address(ep: Endpoint) -> Result<String> {
    API_EP.get(&ep)
          .map(|a| format!("{}{}", API_BASE, a))
          .ok_or(Error::Other("Attempted to use unexistent endpoint"))
}

mod filters_to_string {
    use ::*;

    pub fn match_filter(f: MatchFilter) -> String {
        let mut out = Vec::new();
        match f.featured {
            Some(f) => out.push(format!("featured={}", if f { 1 } else { 0 })),
            None => {},
        }
        match f.has_result {
            Some(r) => out.push(format!("has_result={}", if r { 1 } else { 0 })),
            None => {},
        }
        match f.sort {
            Some(s) => out.push(format!("sort={}", s.to_string())),
            None => {},
        }
        match f.participant_id {
            Some(i) => out.push(format!("participant_id={}", i.0)),
            None => {},
        }
        match f.tournament_ids {
            Some(ref i) => out.push(format!("tournament_ids={}",
                                            i.iter()
                                             .map(|i| i.0.as_str())
                                             .collect::<Vec<&str>>()
                                             .join(","))),
            None => {},
        }
        out.push(format!("with_games={}", if f.with_games { 1 } else { 0 }));
        match f.before_date {
            Some(d) => out.push(format!("before_date={}", d)),
            None => {},
        }
        match f.after_date {
            Some(d) => out.push(format!("after_date={}", d)),
            None => {},
        }
        match f.page {
            Some(p) => out.push(format!("page={}", p)),
            None => {},
        }
        out.join("&")
    }

    pub fn tournament_participants(f: TournamentParticipantsFilter) -> String {
        format!("with_lineup={}&with_custom_fields={}&sort={}&page={}",
                f.with_lineup as u64,
                f.with_custom_fields as u64,
                f.sort.to_string(),
                f.page)
    }

    pub fn tournament_videos(f: TournamentVideosFilter) -> String {
        let mut out = Vec::new();
        match f.category {
            Some(c) => out.push(format!("category={}", c.to_string())),
            None => {},
        }
        out.push(format!("sort={}", f.sort.to_string()));
        match f.page {
            Some(p) => out.push(format!("page={}", p)),
            None => {},
        }
        out.join("&")
    }
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
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET");
    /// assert!(t.is_ok());
    /// ```
    pub fn with_application<S: Into<String>>(api_token: S,
                                             client_id: S,
                                             client_secret: S) -> Result<Toornament> {

        let client = reqwest::Client::new()?;
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

    /// Consumes `Toornament` object and sets timeout to it
    pub fn timeout(mut self, seconds: u64) -> Toornament {
        use std::time::Duration;

        self.client.timeout(Duration::from_secs(seconds));
        self
    }

    /// [Returns either a collection of disciplines]
    /// (https://developer.toornament.com/doc/disciplines#get:disciplines) if id is None or
    /// [a disciplines with the detail of his features]
    /// (https://developer.toornament.com/doc/disciplines#get:disciplines:id)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Getting all disciplines
    /// let all_disciplines: Disciplines = t.disciplines(None).unwrap();
    /// // Get discipline by it's id
    /// let wwe2k17_discipline = t.disciplines(Some(DisciplineId("wwe2k17".to_owned()))).unwrap();
    /// assert_eq!(wwe2k17_discipline.0.len(), 1);
    /// assert_eq!(wwe2k17_discipline.0.first().unwrap().id,
    /// DisciplineId("wwe2k17".to_owned()));
    /// ```
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
        let response = request!(self, get, &address)?;
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
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Getting all tournaments
    /// let all_tournaments: Tournaments = t.tournaments(None, true).unwrap();
    /// // Get tournament by it's id
    /// let tournament = t.tournaments(Some(TournamentId("1".to_owned())), true).unwrap();
    /// assert_eq!(tournament.0.len(), 1);
    /// assert_eq!(tournament.0.first().unwrap().id,
    /// Some(TournamentId("1".to_owned())));
    /// ```
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
        let response = request!(self, get, &address)?;
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
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get tournament by it's id
    /// let tournaments = t.tournaments(Some(TournamentId("1".to_owned())), true).unwrap();
    /// assert_eq!(tournaments.0.len(), 1);
    /// let mut tournament = tournaments.0.first().unwrap().clone();
    /// assert_eq!(tournament.id, Some(TournamentId("1".to_owned())));
    /// tournament.website(Some("https://toornament.com".to_owned()));
    /// // Editing tournament by calling the appropriate method
    /// let tournament = t.edit_tournament(tournament.clone()).unwrap();
    /// assert_eq!(tournament.website,
    /// Some("https://toornament.com".to_owned()));
    /// ```
    pub fn edit_tournament(&self,
                           tournament: Tournament) -> Result<Tournament> {
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
            debug!("Editing tournament: {:#?}", tournament);
            response = request_body!(self, patch, &address, body.as_str())?;

        } else {
            debug!("Creating tournament: {:#?}", tournament);
            response = request_body!(self, post, &address, body.as_str())?;
        }
        Ok(serde_json::from_reader(response)?)
    }

    /// [Deletes a tournament, its participants and all its matches]
    /// (https://developer.toornament.com/doc/tournaments#delete:tournaments:id).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Deleting tournament with id = "1"
    /// assert!(t.delete_tournament(TournamentId("1".to_owned())).is_ok());
    /// ```
    pub fn delete_tournament(&self,
                             id: TournamentId) -> Result<()> {
        debug!("Deleting tournament by id: {:?}", id);
        let address = format!("{}/{}",
                              get_ep_address(Endpoint::PublicTournaments)?,
                              id.0);
        let _ = request!(self, delete, &address)?;
        Ok(())
    }

    /// [Returns the private and public tournaments on which the authenticated user has access.
    /// The result is filtered, sorted and paginated by the given query parameters. A maximum of
    /// 50 tournaments is returned (per page).]
    /// (https://developer.toornament.com/doc/tournaments#get:metournaments)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get all my tournaments
    /// let tournaments = t.my_tournaments().unwrap();
    /// ```
    pub fn my_tournaments(&self) -> Result<Tournaments> {
        debug!("Getting all tournaments");
        let address = get_ep_address(Endpoint::MyTournaments)?;
        let response = request!(self, get, &address)?;
        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of matches from one tournament. The collection may be filtered and
    /// sorted by optional query parameters. The tournament must be public to have access to its
    /// matches, meaning the tournament organizer has published it.]
    /// (https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get all matches of a tournament with id = "1"
    /// let matches = t.matches(TournamentId("1".to_owned()), None, true).unwrap();
    /// // Get match with match id = "2" of a tournament with id = "1"
    /// let matches = t.matches(TournamentId("1".to_owned()), Some(MatchId("2".to_owned())), true).unwrap();
    /// ```
    pub fn matches(&self, id: TournamentId, match_id: Option<MatchId>, with_games: bool)
        -> Result<Matches> {
        let response = match match_id {
            Some(match_id) => {
                debug!("Getting matches by tournament id and match id: {:?} / {:?}", id, match_id);
                let ep = format!("{}/{}?with_games={}",
                                 get_ep_address(Endpoint::Matches)?,
                                 match_id.0,
                                 if with_games { "1" } else { "0" });
                let address = ep.replace(":tournament_id:", &id.0);
                request!(self, get, &address)?
            },
            None => {
                debug!("Getting matches by tournament id: {:?}", id);
                let ep = format!("{}?with_games={}",
                                 get_ep_address(Endpoint::Matches)?,
                                 if with_games { "1" } else { "0" });
                let address = ep.replace(":tournament_id:", &id.0);
                request!(self, get, &address)?
            },
        };

        Ok(serde_json::from_reader(response)?)
    }

    /// [Retrieve a collection of matches from a specific discipline, filtered and sorted by the
    /// given query parameters. It might be a list of matches from different tournaments, but only
    /// from public tournaments. The matches are returned by 20.]
    /// (https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get all matches by a discipline with id = "1" with default filter
    /// let matches = t.matches_by_discipline(DisciplineId("1".to_owned()), MatchFilter::default()).unwrap();
    /// ```
    pub fn matches_by_discipline(&self, id: DisciplineId, filter: MatchFilter)
        -> Result<Matches> {
        debug!("Getting matches by tournament id: {:?}", id);
        let ep = format!("{}?{}",
                         get_ep_address(Endpoint::Matches)?,
                         filters_to_string::match_filter(filter));
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [If you need to make changes on your match data, you are able to do so by patching one or
    /// several fields of your match.](https://developer.toornament.com/doc/matches#patch:tournaments:tournament_id:matches:id)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get a match with id = "2" of a tournament with id = "1"
    /// let matches = t.matches(TournamentId("1".to_owned()),
    ///                         Some(MatchId("2".to_owned())),
    ///                         true).unwrap();
    /// let mut match_to_edit = matches.0.first().unwrap().clone();
    /// match_to_edit.number(2u64);
    /// match_to_edit = t.update_match(TournamentId("1".to_owned()),
    ///                                MatchId("2".to_owned()),
    ///                                match_to_edit).unwrap();
    /// assert_eq!(match_to_edit.number, 2u64);
    /// ```
    pub fn update_match(&self,
                        id: TournamentId,
                        match_id: MatchId,
                        updated_match: Match) -> Result<Match> {
        debug!("Updating a match by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let ep = format!("{}/{}", get_ep_address(Endpoint::Matches)?, match_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&updated_match)?;
        let response = request_body!(self, patch, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed result about one match.]
    /// (https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches:id:result)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get a match result of a match with id = "2" of a tournament with id = "1"
    /// let result = t.match_result(TournamentId("1".to_owned()),
    ///                             MatchId("2".to_owned())).unwrap();
    /// ```
    pub fn match_result(&self, id: TournamentId, match_id: MatchId) -> Result<MatchResult> {
        debug!("Getting match result by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let ep = format!("{}/{}/result", get_ep_address(Endpoint::Matches)?, match_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update or create detailed result about one match.]
    /// (https://developer.toornament.com/doc/matches#put:tournaments:tournament_id:matches:id:result)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Define a result
    /// let result = MatchResult {
    ///     status: MatchStatus::Completed,
    ///     opponents: Opponents::default(),
    /// };
    /// // Set match result for a match with id = "2" of a tournament with id = "1"
    /// assert!(t.set_match_result(TournamentId("1".to_owned()),
    ///                            MatchId("2".to_owned()),
    ///                            result).is_ok());
    /// ```
    pub fn set_match_result(&self,
                            id: TournamentId,
                            match_id: MatchId,
                            result: MatchResult) -> Result<MatchResult> {
        debug!("Setting match result by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let ep = format!("{}/{}/result", get_ep_address(Endpoint::Matches)?, match_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&result)?;
        let response = request_body!(self, put, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of games from one match.]
    /// (https://developer.toornament.com/doc/games#get:tournaments:tournament_id:matches:match_id:games)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get match games of a match with id = "2" of a tournament with id = "1"
    /// let games = t.match_games(TournamentId("1".to_owned()),
    ///                           MatchId("2".to_owned()),
    ///                           true).unwrap();
    /// ```
    pub fn match_games(&self,
                       id: TournamentId,
                       match_id: MatchId,
                       with_stats: bool) -> Result<Games> {
        debug!("Getting match games by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let ep = format!("{}/{}/games?with_stats={}",
                         get_ep_address(Endpoint::Matches)?,
                         match_id.0,
                         if with_stats { 1 } else { 0 });
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;
        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed information about one game.]
    /// (https://developer.toornament.com/doc/games?#get:tournaments:tournament_id:matches:match_id:games:number)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get a match game with number "3" of a match with id = "2" of a tournament with id = "1"
    /// let game = t.match_game(TournamentId("1".to_owned()),
    ///                         MatchId("2".to_owned()),
    ///                         GameNumber(3i64),
    ///                         true).unwrap();
    /// ```
    pub fn match_game(&self,
                      id: TournamentId,
                      match_id: MatchId,
                      game_number: GameNumber,
                      with_stats: bool) -> Result<Game> {
        debug!("Getting match game in details by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let ep = format!("{}/{}/games/{}?with_stats={}",
                         get_ep_address(Endpoint::Matches)?,
                         match_id.0,
                         game_number.0,
                         if with_stats { 1 } else { 0 });
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [If you need to make changes on your game data, you are able to do so by patching one
    /// or several fields of your game.]
    /// (https://developer.toornament.com/doc/games?#patch:tournaments:tournament_id:matches:match_id:games:number)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// let game = Game {
    ///     number: GameNumber(3i64),
    ///     status: MatchStatus::Completed,
    ///     opponents: Opponents::default(),
    /// };
    /// // Update a match game with number "3" of a match with id = "2" of a tournament with id = "1"
    /// assert!(t.update_match_game(TournamentId("1".to_owned()),
    ///                             MatchId("2".to_owned()),
    ///                             GameNumber(3i64),
    ///                             game).is_ok());
    /// ```
    pub fn update_match_game(&self,
                             id: TournamentId,
                             match_id: MatchId,
                             game_number: GameNumber,
                             game: Game) -> Result<Game> {
        debug!("Updating match game by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let ep = format!("{}/{}/games/{}",
                         get_ep_address(Endpoint::Matches)?,
                         match_id.0,
                         game_number.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&game)?;
        let response = request_body!(self, patch, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed result about one specific game.]
    /// (https://developer.toornament.com/doc/games?#get:tournaments:tournament_id:matches:match_id:games:number:result)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get a match game result with number "3" of a match with id = "2" of a tournament with id = "1"
    /// assert!(t.match_game_result(TournamentId("1".to_owned()),
    ///                             MatchId("2".to_owned()),
    ///                             GameNumber(3i64)).is_ok());
    /// ```
    pub fn match_game_result(&self,
                             id: TournamentId,
                             match_id: MatchId,
                             game_number: GameNumber) -> Result<MatchResult> {
        debug!("Getting match game result by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let ep = format!("{}/{}/games/{}/result",
                         get_ep_address(Endpoint::Matches)?,
                         match_id.0,
                         game_number.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Updates or creates detailed result about one game.]
    /// (https://developer.toornament.com/doc/games?#put:tournaments:tournament_id:matches:match_id:games:number:result)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Define a result
    /// let result = MatchResult {
    ///     status: MatchStatus::Completed,
    ///     opponents: Opponents::default(),
    /// };
    /// // Update a match game result with number "3" of a match with id = "2" of a tournament with id = "1"
    /// assert!(t.update_match_game_result(TournamentId("1".to_owned()),
    ///                                    MatchId("2".to_owned()),
    ///                                    GameNumber(3i64),
    ///                                    result,
    ///                                    true).is_ok());
    /// ```
    pub fn update_match_game_result(&self,
                                    id: TournamentId,
                                    match_id: MatchId,
                                    game_number: GameNumber,
                                    result: MatchResult,
                                    update_match: bool) -> Result<MatchResult> {
        debug!("Setting match game result by tournament id and match id: {:?} / {:?}",
               id,
               match_id);
        let ep = format!("{}/{}/games/{}/result?update_match={}",
                         get_ep_address(Endpoint::Matches)?,
                         match_id.0,
                         game_number.0,
                         if update_match { 1 } else { 0 });
        let address = ep.replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&result)?;
        let response = request_body!(self, put, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of participants from one tournament. The tournament must be public
    /// to have access to its participants, meaning the tournament organizer has published it. The
    /// participants are returned by 256.]
    /// (https://developer.toornament.com/doc/participant#get:tournaments:tournament_id:participants)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get participants of a tournament with id = "1" with default filter
    /// let participants = t.tournament_participants(
    ///     TournamentId("1".to_owned()),
    ///     TournamentParticipantsFilter::default()).unwrap();
    /// ```
    pub fn tournament_participants(&self,
                                   id: TournamentId,
                                   filter: TournamentParticipantsFilter) -> Result<Participants> {
        debug!("Getting tournament participants by tournament id: {:?}", id);
        let ep = format!("{}?{}",
                         get_ep_address(Endpoint::Participants)?,
                         filters_to_string::tournament_participants(filter));
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a participant in a tournament.]
    /// (https://developer.toornament.com/doc/participants?#post:tournaments:tournament_id:participants)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Define a participant
    /// let participant = Participant::create("Test participant");
    /// // Create a participant for a tournament with id = "1"
    /// let participant = t.create_tournament_participant(TournamentId("1".to_owned()),
    ///                                                   participant).unwrap();
    /// assert!(participant.id.is_some());
    /// ```
    pub fn create_tournament_participant(&self,
                                         id: TournamentId,
                                         participant: Participant) -> Result<Participant> {
        debug!("Creating a participant for tournament with id: {:?}", id);
        let address = get_ep_address(Endpoint::Participants)?
                      .replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&participant)?;
        let response = request_body!(self, post, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a list of participants in a tournament. If any participant already exists he will
    /// be erased.]
    /// (https://developer.toornament.com/doc/participants?_locale=en#put:tournaments:tournament_id:participants)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// let mut participants = vec![Participant::create("First participant"),
    ///                             Participant::create("Second participant")];
    /// // Update a participant for a tournament with id = "1"
    /// let new_participants = t.update_tournament_participants(TournamentId("1".to_owned()),
    ///                                                         Participants(participants)).unwrap();
    /// assert_eq!(new_participants.0.len(), 2);
    /// ```
    pub fn update_tournament_participants(&self,
                                          id: TournamentId,
                                          participants: Participants) -> Result<Participants> {
        debug!("Creating a list of participants for tournament with id: {:?}", id);
        let address = get_ep_address(Endpoint::Participants)?
                      .replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&participants)?;
        let response = request_body!(self, put, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed information about one participant.]
    /// (https://developer.toornament.com/doc/participants?_locale=en#get:tournaments:tournament_id:participants:id)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get a participant with id = "2" of a tournament with id = "1"
    /// let participant = t.tournament_participant(TournamentId("1".to_owned()),
    ///                                            ParticipantId("2".to_owned())).unwrap();
    /// assert_eq!(participant.id, Some(ParticipantId("2".to_owned())));
    /// ```
    pub fn tournament_participant(&self,
                                  id: TournamentId,
                                  participant_id: ParticipantId) -> Result<Participant> {
        debug!("Getting tournament participant by tournament id and participant id: {:?} / {:?}",
               id,
               participant_id);
        let ep = format!("{}/{}",
                         get_ep_address(Endpoint::Participants)?,
                         participant_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update some of the editable information on a participant.]
    /// (https://developer.toornament.com/doc/participants?_locale=en#patch:tournaments:tournament_id:participants:id)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get a participant with id = "2" of a tournament with id = "1"
    /// let mut participant = t.tournament_participant(TournamentId("1".to_owned()),
    ///                                                ParticipantId("2".to_owned())).unwrap();
    /// assert_eq!(participant.id, Some(ParticipantId("2".to_owned())));
    /// // Update the participant's name and send it
    /// participant.name("Updated participant name here".to_owned());
    /// let updated_participant = t.update_tournament_participant(
    ///     TournamentId("1".to_owned()),
    ///     ParticipantId("2".to_owned()),
    ///     participant).unwrap();
    /// assert_eq!(updated_participant.id, Some(ParticipantId("2".to_owned())));
    /// assert_eq!(updated_participant.name, "Updated participant name here");
    /// ```
    pub fn update_tournament_participant(&self,
                                         id: TournamentId,
                                         participant_id: ParticipantId,
                                         participant: Participant) -> Result<Participant> {
        debug!("Updating a participant for tournament with id and participant id: {:?} / {:?}",
               id,
               participant_id);
        let ep = format!("{}/{}",
                         get_ep_address(Endpoint::Participants)?,
                         participant_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&participant)?;
        let response = request_body!(self, patch, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Deletes one participant.]
    /// (https://developer.toornament.com/doc/participants?_locale=en#delete:tournaments:tournament_id:participants:id)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Delete a participant with id = "2" of a tournament with id = "1"
    /// assert!(t.delete_tournament_participant(TournamentId("1".to_owned()),
    ///                                         ParticipantId("2".to_owned())).is_ok());
    /// ```
    pub fn delete_tournament_participant(&self,
                                         id: TournamentId,
                                         participant_id: ParticipantId) -> Result<()> {
        debug!("Deleting a participant for tournament with id and participant id: {:?} / {:?}",
               id,
               participant_id);
        let ep = format!("{}/{}",
                         get_ep_address(Endpoint::Participants)?,
                         participant_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, delete, &address)?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Other("Something went wrong"))
        }
    }

    /// [Returns a collection of permission from one tournament.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#get:tournaments:tournament_id:permissions)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get permissions of a tournament with id = "1"
    /// let permissions = t.tournament_permissions(TournamentId("1".to_owned())).unwrap();
    /// ```
    pub fn tournament_permissions(&self, id: TournamentId) -> Result<Permissions> {
        debug!("Getting tournament permissions by tournament id: {:?}", id);
        let address = get_ep_address(Endpoint::Permissions)?.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a permission for a user on a tournament.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#post:tournaments:tournament_id:permissions)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// use std::collections::BTreeSet;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Define our permission
    /// let mut attributes = BTreeSet::new();
    /// attributes.insert(PermissionAttribute::Register);
    /// attributes.insert(PermissionAttribute::Edit);
    ///
    /// let permission = Permission::create("test@mail.ru", PermissionAttributes(attributes));
    /// // Add permission to a tournament with id = "1"
    /// let new_permission = t.create_tournament_permission(TournamentId("1".to_owned()),
    ///                                                     permission).unwrap();
    /// assert!(new_permission.id.is_some());
    /// assert_eq!(new_permission.email, "test@mail.ru");
    /// assert_eq!(new_permission.attributes.0.len(), 2);
    /// ```
    pub fn create_tournament_permission(&self,
                                        id: TournamentId,
                                        permission: Permission) -> Result<Permission> {
        debug!("Creating tournament permissions by tournament id: {:?}", id);
        let address = get_ep_address(Endpoint::Permissions)?.replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&permission)?;
        let response = request_body!(self, get, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Retrieves a permission of a tournament.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#get:tournaments:tournament_id:permissions:permission_id)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// use std::collections::BTreeSet;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get a permission with id = "2" of a tournament with id = "1"
    /// let permission = t.tournament_permission(TournamentId("1".to_owned()),
    ///                                          PermissionId("2".to_owned())).unwrap();
    /// assert_eq!(permission.id, Some(PermissionId("2".to_owned())));
    /// ```
    pub fn tournament_permission(&self,
                                 id: TournamentId,
                                 permission_id: PermissionId) -> Result<Permission> {
        debug!("Getting tournament permission by tournament id and permission id: {:?} / {:?}",
               id,
               permission_id);
        let ep = format!("{}/{}",
                         get_ep_address(Endpoint::Permissions)?,
                         permission_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update rights of a permission.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#patch:tournaments:tournament_id:permissions:permission_id)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// use std::collections::BTreeSet;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Define our permission attributes
    /// let mut attributes = BTreeSet::new();
    /// attributes.insert(PermissionAttribute::Register);
    /// attributes.insert(PermissionAttribute::Edit);
    ///
    /// // Update attributes of a permission with id = "2" of a tournament with id = "1"
    /// let permission = t.update_tournament_permission_attributes(
    ///     TournamentId("1".to_owned()),
    ///     PermissionId("2".to_owned()),
    ///     PermissionAttributes(attributes)).unwrap();
    /// assert_eq!(permission.id, Some(PermissionId("2".to_owned())));
    /// assert_eq!(permission.attributes.0.len(), 2);
    /// assert!(permission.attributes.0.iter().find(|p| *p == &PermissionAttribute::Edit).is_some());
    /// assert!(permission.attributes.0.iter().find(|p| *p == &PermissionAttribute::Register).is_some());
    /// ```
    pub fn update_tournament_permission_attributes(&self,
                                                   id: TournamentId,
                                                   permission_id: PermissionId,
                                                   attributes: PermissionAttributes)
        -> Result<Permission> {
        #[derive(Serialize)]
        struct WrappedAttributes {
            attributes: PermissionAttributes,
        }
        debug!("Updating tournament permission attributes by tournament id \
               and permission id: {:?} / {:?}",
               id,
               permission_id);
        let ep = format!("{}/{}",
                         get_ep_address(Endpoint::Permissions)?,
                         permission_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let wrapped_attributes = WrappedAttributes { attributes: attributes };
        let body = serde_json::to_string(&wrapped_attributes)?;
        let response = request_body!(self, patch, &address, body.as_str())?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Delete a user permission of a tournament.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#delete:tournaments:tournament_id:permissions:permission_id)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// use std::collections::BTreeSet;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Delete a permission with id = "2" of a tournament with id = "1"
    /// assert!(t.delete_tournament_permission(
    ///     TournamentId("1".to_owned()),
    ///     PermissionId("2".to_owned())).is_ok());
    /// ```
    pub fn delete_tournament_permission(&self,
                                        id: TournamentId,
                                        permission_id: PermissionId) -> Result<()> {
        debug!("Deleting a permission for tournament with id and permission id: {:?} / {:?}",
               id,
               permission_id);
        let ep = format!("{}/{}",
                         get_ep_address(Endpoint::Permissions)?,
                         permission_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, delete, &address)?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Other("Something went wrong"))
        }
    }

    /// [Returns a collection of stages from one tournament. The tournament must be public to have
    /// access to its stages, meaning the tournament organizer must publish it.]
    /// (https://developer.toornament.com/doc/stages?_locale=en#get:tournaments:tournament_id:stages)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// use std::collections::BTreeSet;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get stages of a tournament with id = "1"
    /// let stages = t.tournament_stages(TournamentId("1".to_owned())).unwrap();
    /// ```
    pub fn tournament_stages(&self, id: TournamentId) -> Result<Stages> {
        debug!("Getting tournament stages by tournament id: {:?}", id);
        let address = get_ep_address(Endpoint::Stages)?.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of videos from one tournament. The collection may be filtered and
    /// sorted by optional query parameters. The tournament must be public to have access to its
    /// videos, meaning the tournament organizer has published it. The videos are returned by 20.]
    /// (https://developer.toornament.com/doc/videos?_locale=en#get:tournaments:tournament_id:videos)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use toornament::*;
    /// use std::collections::BTreeSet;
    /// let t = Toornament::with_application("API_TOKEN",
    ///                                      "CLIENT_ID",
    ///                                      "CLIENT_SECRET").unwrap();
    /// // Get videos of a tournament with id = "1" with default filter
    /// let videos = t.tournament_videos(TournamentId("1".to_owned()),
    ///                                  TournamentVideosFilter::default()).unwrap();
    /// ```
    pub fn tournament_videos(&self,
                             id: TournamentId,
                             filter: TournamentVideosFilter) -> Result<Videos> {
        debug!("Getting tournament videos by tournament id: {:?}", id);
        let ep = format!("{}?{}",
                         get_ep_address(Endpoint::Videos)?,
                         filters_to_string::tournament_videos(filter));
        let address = ep.replace(":tournament_id:", &id.0);
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }
}

#[cfg(test)]
mod tests {
    use ::MatchFilter;
    use ::filters_to_string;

    #[test]
    fn test_match_filter_to_get_string() {
        let mut f = MatchFilter::default();
        f.featured(true).has_result(true).page(2i64);
        assert_eq!(filters_to_string::match_filter(f),
                   "featured=1&has_result=1&sort=date_asc&with_games=0&page=2");
    }
}
