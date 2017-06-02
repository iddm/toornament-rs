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
mod permissions;
mod stages;
mod common;

pub use error::{ Result, Error };
pub use common::{ TeamSize, Opponent, Opponents, MatchResultSimple, Date };
pub use matches::{ Match, MatchId, Matches, MatchType, MatchResult, MatchStatus };
pub use games::{ GameNumber, Game, Games };
pub use stages::{ StageNumber, StageType, Stage, Stages };
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
pub use filters::{ DateSortFilter, MatchFilter, TournamentParticipantsFilter };
pub use tournaments::{
    TournamentId,
    Tournament,
    Tournaments,
    TournamentStatus,
};


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
    API_EP.get(&ep)
          .map(|a| format!("{}{}", API_BASE, a))
          .ok_or(Error::Other("Attempted to use unexistent endpoint"))
}

mod filters_to_string {
    use ::*;

    pub fn match_filter(f: Option<MatchFilter>) -> String {
        let f = match f {
            Some(f) => f,
            None => return String::default(),
        };
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
        let ep = format!("{}?{}",
                         get_ep_address(Endpoint::Matches)?,
                         filters_to_string::match_filter(filter));
        let address = ep.replace(":tournament_id:", &id.0);
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
        let ep = format!("{}/{}", get_ep_address(Endpoint::Matches)?, match_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
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
        let ep = format!("{}/{}/result", get_ep_address(Endpoint::Matches)?, match_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
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
        let ep = format!("{}/{}/result", get_ep_address(Endpoint::Matches)?, match_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&result)?;
        let response = retry(|| self.client.put(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of games from one match.]
    /// (https://developer.toornament.com/doc/games#get:tournaments:tournament_id:matches:match_id:games)
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
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed information about one game.]
    /// (https://developer.toornament.com/doc/games?#get:tournaments:tournament_id:matches:match_id:games:number)
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
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [If you need to make changes on your game data, you are able to do so by patching one
    /// or several fields of your game.]
    /// (https://developer.toornament.com/doc/games?#patch:tournaments:tournament_id:matches:match_id:games:number)
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
        let response = retry(|| self.client.patch(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed result about one specific game.]
    /// (https://developer.toornament.com/doc/games?#get:tournaments:tournament_id:matches:match_id:games:number:result)
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
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Updates or creates detailed result about one game.]
    /// (https://developer.toornament.com/doc/games?#put:tournaments:tournament_id:matches:match_id:games:number:result)
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
        let response = retry(|| self.client.put(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of participants from one tournament. The tournament must be public
    /// to have access to its participants, meaning the tournament organizer has published it. The
    /// participants are returned by 256.]
    /// (https://developer.toornament.com/doc/participant#get:tournaments:tournament_id:participants)
    pub fn tournament_participants(&self,
                                   id: TournamentId,
                                   filter: TournamentParticipantsFilter) -> Result<Participants> {
        debug!("Getting tournament participants by tournament id: {:?}", id);
        let ep = format!("{}?{}",
                         get_ep_address(Endpoint::Participants)?,
                         filters_to_string::tournament_participants(filter));
        let address = ep.replace(":tournament_id:", &id.0);
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a participant in a tournament.]
    /// (https://developer.toornament.com/doc/participants?#post:tournaments:tournament_id:participants)
    pub fn create_tournament_participant(&self,
                                         id: TournamentId,
                                         participant: Participant) -> Result<Participant> {
        debug!("Creating a participant for tournament with id: {:?}", id);
        let address = get_ep_address(Endpoint::Participants)?
                      .replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&participant)?;
        let response = retry(|| self.client.post(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a list of participants in a tournament. If any participant already exists he will
    /// be erased.]
    /// (https://developer.toornament.com/doc/participants?_locale=en#put:tournaments:tournament_id:participants)
    pub fn update_tournament_participants(&self,
                                          id: TournamentId,
                                          participants: Participants) -> Result<Participants> {
        debug!("Creating a list of participants for tournament with id: {:?}", id);
        let address = get_ep_address(Endpoint::Participants)?
                      .replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&participants)?;
        let response = retry(|| self.client.put(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed information about one participant.]
    /// (https://developer.toornament.com/doc/participants?_locale=en#get:tournaments:tournament_id:participants:id)
    pub fn get_tournament_participant(&self,
                                      id: TournamentId,
                                      participant_id: ParticipantId) -> Result<Participant> {
        debug!("Getting tournament participant by tournament id and participant id: {:?} / {:?}",
               id,
               participant_id);
        let ep = format!("{}/{}",
                         get_ep_address(Endpoint::Participants)?,
                         participant_id.0);
        let address = ep.replace(":tournament_id:", &id.0);
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone())))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update some of the editable information on a participant.]
    /// (https://developer.toornament.com/doc/participants?_locale=en#patch:tournaments:tournament_id:participants:id)
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
        let response = retry(|| self.client.patch(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Deletes one participant.]
    /// (https://developer.toornament.com/doc/participants?_locale=en#delete:tournaments:tournament_id:participants:id)
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
        let response = retry(|| self.client.delete(&address)
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Other("Something went wrong"))
        }
    }

    /// [Returns a collection of permission from one tournament.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#get:tournaments:tournament_id:permissions)
    pub fn tournament_permissions(&self, id: TournamentId) -> Result<Permissions> {
        debug!("Getting tournament permissions by tournament id: {:?}", id);
        let address = get_ep_address(Endpoint::Permissions)?.replace(":tournament_id:", &id.0);
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a permission for a user on a tournament.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#post:tournaments:tournament_id:permissions)
    pub fn create_tournament_permission(&self,
                                        id: TournamentId,
                                        permission: Permission) -> Result<Permission> {
        debug!("Creating tournament permissions by tournament id: {:?}", id);
        let address = get_ep_address(Endpoint::Permissions)?.replace(":tournament_id:", &id.0);
        let body = serde_json::to_string(&permission)?;
        let response = retry(|| self.client.get(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Retrieves a permission of a tournament.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#get:tournaments:tournament_id:permissions:permission_id)
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
        let response = retry(|| self.client.get(&address)
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update rights of a permission.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#patch:tournaments:tournament_id:permissions:permission_id)
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
        let response = retry(|| self.client.patch(&address)
                                           .body(body.as_str())
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Delete a user permission of a tournament.]
    /// (https://developer.toornament.com/doc/permissions?_locale=en#delete:tournaments:tournament_id:permissions:permission_id)
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
        let response = retry(|| self.client.delete(&address)
                                           .header(XApiKey(self.keys.0.clone()))
                                           .header(Authorization(Bearer { token: self.oauth_token.clone() })))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Other("Something went wrong"))
        }
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
        assert_eq!(filters_to_string::match_filter(None), "");
        assert_eq!(filters_to_string::match_filter(Some(f)), "featured=1&has_result=1&sort=date_asc&with_games=0&page=2");
    }
}
