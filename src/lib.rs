//! Client library for the [Toornament](<https://www.toornament.com>) web API.
//!
//! Log in to Toornament with `Toornament::with_application`.
//! Call API methods to interact with the service directly or user an iterator-like interface to
//! work with it in more rust-idiomatic way.
//!
//! For Toornament API documentation [look here](<https://developer.toornament.com/overview/get-started>).
//!
//! For examples, see the `examples` directory in the source tree.
//!
//! For more readings, look at the [`toornament-rs book`](<https://vityafx.github.io/toornament-rs>).
//!
//! # Usage
//!
//! Start by creating and instance `Toornament` structure and then perform the requests:
//!
//! ```rust,no_run
//! use toornament::*;
//!
//! let toornament = Toornament::with_application("API_TOKEN",
//!                                               "CLIENT_ID",
//!                                               "CLIENT_SECRET").unwrap();
//! println!("Disciplines: {:?}", toornament.disciplines(None));
//! println!("Disciplines: {:?}", toornament.disciplines_iter()
//!                                         .all()
//!                                         .collect::<Disciplines>());
//! ```
//!
//! # Additional notes
//! The `Toornament` structure is `Send` and `Sync`, so it can be simply shared among
//! threads. Also, the `Toornament` objects may live as long as you need to: the object will
//! refresh it's access token once it is expired, so you may just create it once and use
//! everywhere.
#![warn(missing_docs)]
#![deny(warnings)]

use std::io::Read;
use std::sync::Mutex;

#[macro_use]
mod macroses;
mod common;
mod disciplines;
mod endpoints;
mod error;
mod filters;
mod games;
pub mod info;
pub mod iter;
mod matches;
mod opponents;
mod participants;
mod permissions;
mod stages;
mod streams;
mod tournaments;
mod videos;

pub use common::{Date, MatchResultSimple, TeamSize};
pub use disciplines::{AdditionalFields, Discipline, DisciplineId, Disciplines};
use endpoints::Endpoint;
pub use error::{
    Error, IterError, Result, ToornamentError, ToornamentErrorScope, ToornamentErrorType,
    ToornamentErrors, ToornamentServiceError,
};
pub use filters::{
    CreateDateSortFilter, DateSortFilter, MatchFilter, TournamentParticipantsFilter,
    TournamentVideosFilter,
};
pub use games::{Game, GameNumber, Games};
pub use iter::*;
pub use matches::{Match, MatchFormat, MatchId, MatchResult, MatchStatus, MatchType, Matches};
pub use opponents::{Opponent, Opponents};
pub use participants::{
    CustomField, CustomFieldType, CustomFields, Participant, ParticipantId, ParticipantLogo,
    ParticipantType, Participants,
};
pub use permissions::{
    Permission, PermissionAttribute, PermissionAttributes, PermissionId, Permissions,
};
pub use stages::{Stage, StageNumber, StageType, Stages};
pub use streams::{Stream, StreamId, Streams};
pub use tournaments::{Tournament, TournamentId, TournamentStatus, Tournaments};
pub use videos::{Video, VideoCategory, Videos};

/// Create the request builer.
macro_rules! build_request {
    ($toornament:ident, $method:ident, $address:expr) => {{
        $toornament
            .client
            .$method($address)
            .header("X-Api-Key", $toornament.keys.0.clone())
            .bearer_auth(&$toornament.fresh_token()?)
    }};
}

/// Macro only for internal use with the `Toornament` object (relies on it's fields)
macro_rules! request {
    ($toornament:ident, $method:ident, $address:expr) => {{
        build_request!($toornament, $method, $address).send()
    }};
}

/// Macro only for internal use with the `Toornament` object (relies on it's fields)
macro_rules! request_body {
    ($toornament:ident, $method:ident, $address:expr, $body:expr) => {{
        build_request!($toornament, $method, $address)
            .body($body)
            .send()
    }};
}

#[derive(Debug, Clone)]
struct AccessToken {
    access_token: String,
    expires: u64,
}

fn parse_token<R: Read>(json_str: R) -> Result<AccessToken> {
    #[derive(Debug, Clone, serde::Deserialize)]
    struct OauthAccessToken {
        access_token: String,
        expires_in: u64,
    }

    let oauth = serde_json::from_reader::<_, OauthAccessToken>(json_str)?;
    Ok(AccessToken {
        access_token: oauth.access_token,
        expires: chrono::Local::now().timestamp() as u64 + oauth.expires_in,
    })
}

fn authenticate(
    client: &reqwest::blocking::Client,
    client_id: &str,
    client_secret: &str,
) -> Result<AccessToken> {
    use std::collections::HashMap;

    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    parse_token(
        client
            .post(&Endpoint::OauthToken.to_string())
            .form(&params)
            .send()?,
    )
}

/// Main structure. Should be your point of start using the service.
/// This struct covers all the `toornament` API.
#[derive(Debug)]
pub struct Toornament {
    client: reqwest::blocking::Client,
    keys: (String, String, String),
    oauth_token: Mutex<AccessToken>,
}
impl Toornament {
    /// Returns currently stored token
    fn current_token(&self) -> Result<String> {
        match self.oauth_token.lock() {
            Ok(g) => Ok(g.access_token.to_owned()),
            Err(_) => Err(Error::Rest("Can't get the token")),
        }
    }

    /// Always returns fresh token (refreshes it if neeeded)
    fn fresh_token(&self) -> Result<String> {
        let mut need_refresh = false;
        {
            let access_token = match self.oauth_token.lock() {
                Ok(g) => g,
                Err(_) => return Err(Error::Rest("Can't get the token")),
            };
            if chrono::Local::now().timestamp() as u64 > access_token.expires {
                need_refresh = true;
            }
        }
        if need_refresh && !self.refresh() {
            return Err(Error::Rest("Could not refresh the token"));
        }

        self.current_token()
    }

    /// Creates new `Toornament` object with client credentials
    /// which is your user API_Token, application's client id and secret.
    /// You may obtain application's credentials [here](<https://developer.toornament.com/applications/>)
    /// (You must be logged in to open the page).
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
    pub fn with_application<S: Into<String>>(
        api_token: S,
        client_id: S,
        client_secret: S,
    ) -> Result<Toornament> {
        let client = reqwest::blocking::Client::new();
        let keys = (api_token.into(), client_id.into(), client_secret.into());
        let token = authenticate(&client, &keys.1, &keys.2)?;

        Ok(Toornament {
            client,
            keys,
            oauth_token: Mutex::new(token),
        })
    }

    /// Refreshes the oauth token. Automatically used when it is expired.
    pub fn refresh(&self) -> bool {
        let mut g = match self.oauth_token.lock() {
            Ok(g) => g,
            Err(e) => {
                log::error!("Unable to refresh token: {:?}", e);
                return false;
            }
        };

        match authenticate(&self.client, &self.keys.1, &self.keys.2) {
            Ok(token) => {
                *g = token;
                true
            }
            Err(e) => {
                log::error!("Unable to refresh token: {:?}", e);
                false
            }
        }
    }

    /// Consumes `Toornament` object and sets timeout to it
    pub fn timeout(mut self, seconds: u64) -> Result<Toornament> {
        use std::time::Duration;

        self.client = reqwest::blocking::ClientBuilder::new()
            .timeout(Duration::from_secs(seconds))
            .build()?;
        Ok(self)
    }

    /// Returns Iterator-like objects to work with tournaments and it's subobjects.
    pub fn tournaments_iter(&self) -> iter::TournamentsIter {
        iter::TournamentsIter::new(self)
    }

    /// Returns Iterator-like objects to work with disciplines and it's subobjects.
    pub fn disciplines_iter(&self) -> iter::DisciplinesIter {
        iter::DisciplinesIter::new(self)
    }

    /// [Returns either a collection of disciplines](<https://developer.toornament.com/doc/disciplines#get:disciplines>) if id is None or
    /// [a disciplines with the detail of his features](<https://developer.toornament.com/doc/disciplines#get:disciplines:id>)
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
            log::debug!("Getting disciplines with id: {:?}", id);
            address = Endpoint::DisciplineById(id).to_string();
        } else {
            log::debug!("Getting all disciplines");
            address = Endpoint::AllDisciplines.to_string();
        }
        let response = request!(self, get, &address)?;
        if id_is_set {
            Ok(Disciplines(vec![serde_json::from_reader::<_, Discipline>(
                response,
            )?]))
        } else {
            Ok(serde_json::from_reader(response)?)
        }
    }

    /// [Returns a collection of public tournaments filtered and sorted by the given query
    /// parameters. A maximum of 20 tournaments will be returned. Only public tournaments are visible.](<https://developer.toornament.com/doc/tournaments#get:tournaments>) if id is `None` or
    /// [a detailed information about one tournament. The tournament must be public.](<https://developer.toornament.com/doc/tournaments#get:tournaments:id>)
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
    pub fn tournaments(
        &self,
        tournament_id: Option<TournamentId>,
        with_streams: bool,
    ) -> Result<Tournaments> {
        let address;
        let id_is_set = tournament_id.is_some();
        if let Some(tournament_id) = tournament_id {
            log::debug!("Getting tournament with id: {:?}", tournament_id);
            address = Endpoint::TournamentByIdGet {
                tournament_id,
                with_streams,
            }
            .to_string();
        } else {
            log::debug!("Getting all tournaments");
            address = Endpoint::AllTournaments { with_streams }.to_string();
        }
        let response = request!(self, get, &address)?;
        if id_is_set {
            Ok(Tournaments(vec![serde_json::from_reader::<_, Tournament>(
                response,
            )?]))
        } else {
            Ok(serde_json::from_reader(response)?)
        }
    }

    /// [Updates some of the editable information on a tournament.](<https://developer.toornament.com/doc/tournaments#patch:tournaments:id>) if `tournament.id`
    /// is set otherwise [creates a tournament](<https://developer.toornament.com/doc/tournaments#post:tournaments>).
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
    /// tournament = tournament.website(Some("<https://toornament.com>".to_owned()));
    /// // Editing tournament by calling the appropriate method
    /// let tournament = t.edit_tournament(tournament.clone()).unwrap();
    /// assert_eq!(tournament.website,
    /// Some("https://toornament.com".to_owned()));
    /// ```
    pub fn edit_tournament(&self, tournament: Tournament) -> Result<Tournament> {
        let address;
        let id_is_set = tournament.id.is_some();
        if let Some(id) = tournament.id.clone() {
            address = Endpoint::TournamentByIdUpdate(id).to_string();
        } else {
            address = Endpoint::TournamentCreate.to_string();
        }
        let body = serde_json::to_string(&tournament)?;
        let response = if id_is_set {
            log::debug!("Editing tournament: {:#?}", tournament);
            request_body!(self, patch, &address, body)?
        } else {
            log::debug!("Creating tournament: {:#?}", tournament);
            request_body!(self, post, &address, body)?
        };
        Ok(serde_json::from_reader(response)?)
    }

    /// [Deletes a tournament, its participants and all its matches](<https://developer.toornament.com/doc/tournaments#delete:tournaments:id>).
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
    pub fn delete_tournament(&self, id: TournamentId) -> Result<()> {
        log::debug!("Deleting tournament by id: {:?}", id);
        let address = Endpoint::TournamentByIdUpdate(id).to_string();
        let _ = request!(self, delete, &address)?;
        Ok(())
    }

    /// [Returns the private and public tournaments on which the authenticated user has access.
    /// The result is filtered, sorted and paginated by the given query parameters. A maximum of
    /// 50 tournaments is returned (per page).](<https://developer.toornament.com/doc/tournaments#get:metournaments>)
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
        log::debug!("Getting all tournaments");
        let address = Endpoint::MyTournaments.to_string();
        let response = request!(self, get, &address)?;
        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of matches from one tournament. The collection may be filtered and
    /// sorted by optional query parameters. The tournament must be public to have access to its
    /// matches, meaning the tournament organizer has published it.](<https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches>)
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
    pub fn matches(
        &self,
        tournament_id: TournamentId,
        match_id: Option<MatchId>,
        with_games: bool,
    ) -> Result<Matches> {
        let response = match match_id {
            Some(match_id) => {
                log::debug!(
                    "Getting matches by tournament id and match id: {:?} / {:?}",
                    tournament_id,
                    match_id
                );
                let address = Endpoint::MatchByIdGet {
                    tournament_id,
                    match_id,
                    with_games,
                }
                .to_string();
                request!(self, get, &address)?
            }
            None => {
                log::debug!("Getting matches by tournament id: {:?}", tournament_id);
                let address = Endpoint::MatchesByTournament {
                    tournament_id,
                    with_games,
                }
                .to_string();
                request!(self, get, &address)?
            }
        };

        Ok(serde_json::from_reader(response)?)
    }

    /// [Retrieve a collection of matches from a specific discipline, filtered and sorted by the
    /// given query parameters. It might be a list of matches from different tournaments, but only
    /// from public tournaments. The matches are returned by 20.](<https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches>)
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
    pub fn matches_by_discipline(
        &self,
        discipline_id: DisciplineId,
        filter: MatchFilter,
    ) -> Result<Matches> {
        log::debug!("Getting matches by discipline id: {:?}", discipline_id);
        let address = Endpoint::MatchesByDiscipline {
            discipline_id,
            filter,
        }
        .to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [If you need to make changes on your match data, you are able to do so by patching one or
    /// several fields of your match.](<https://developer.toornament.com/doc/matches#patch:tournaments:tournament_id:matches:id>)
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
    /// let mut match_to_edit = matches.0.first().unwrap().clone()
    ///                                .number(2u64);
    /// match_to_edit = t.update_match(TournamentId("1".to_owned()),
    ///                                MatchId("2".to_owned()),
    ///                                match_to_edit).unwrap();
    /// assert_eq!(match_to_edit.number, 2u64);
    /// ```
    pub fn update_match(
        &self,
        tournament_id: TournamentId,
        match_id: MatchId,
        updated_match: Match,
    ) -> Result<Match> {
        log::debug!(
            "Updating a match by tournament id and match id: {:?} / {:?}",
            tournament_id,
            match_id
        );
        let address = Endpoint::MatchByIdUpdate {
            tournament_id,
            match_id,
        }
        .to_string();
        let body = serde_json::to_string(&updated_match)?;
        let response = request_body!(self, patch, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed result about one match.](<https://developer.toornament.com/doc/matches#get:tournaments:tournament_id:matches:id:result>)
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
        log::debug!(
            "Getting match result by tournament id and match id: {:?} / {:?}",
            id,
            match_id
        );
        let address = Endpoint::MatchResult(id, match_id).to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update or create detailed result about one match.](<https://developer.toornament.com/doc/matches#put:tournaments:tournament_id:matches:id:result>)
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
    pub fn set_match_result(
        &self,
        id: TournamentId,
        match_id: MatchId,
        result: MatchResult,
    ) -> Result<MatchResult> {
        log::debug!(
            "Setting match result by tournament id and match id: {:?} / {:?}",
            id,
            match_id
        );
        let address = Endpoint::MatchResult(id, match_id).to_string();
        let body = serde_json::to_string(&result)?;
        let response = request_body!(self, put, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of games from one match.](<https://developer.toornament.com/doc/games#get:tournaments:tournament_id:matches:match_id:games>)
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
    pub fn match_games(
        &self,
        tournament_id: TournamentId,
        match_id: MatchId,
        with_stats: bool,
    ) -> Result<Games> {
        log::debug!(
            "Getting match games by tournament id and match id: {:?} / {:?}",
            tournament_id,
            match_id
        );
        let address = Endpoint::MatchGames {
            tournament_id,
            match_id,
            with_stats,
        }
        .to_string();
        let response = request!(self, get, &address)?;
        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed information about one game.](<https://developer.toornament.com/doc/games?#get:tournaments:tournament_id:matches:match_id:games:number>)
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
    pub fn match_game(
        &self,
        tournament_id: TournamentId,
        match_id: MatchId,
        game_number: GameNumber,
        with_stats: bool,
    ) -> Result<Game> {
        log::debug!(
            "Getting match game in details by tournament id and match id: {:?} / {:?}",
            tournament_id,
            match_id
        );
        let address = Endpoint::MatchGameByNumberGet {
            tournament_id,
            match_id,
            game_number,
            with_stats,
        }
        .to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [If you need to make changes on your game data, you are able to do so by patching one
    /// or several fields of your game.](<https://developer.toornament.com/doc/games?#patch:tournaments:tournament_id:matches:match_id:games:number>)
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
    pub fn update_match_game(
        &self,
        tournament_id: TournamentId,
        match_id: MatchId,
        game_number: GameNumber,
        game: Game,
    ) -> Result<Game> {
        log::debug!(
            "Updating match game by tournament id and match id: {:?} / {:?}",
            tournament_id,
            match_id
        );
        let address = Endpoint::MatchGameByNumberUpdate {
            tournament_id,
            match_id,
            game_number,
        }
        .to_string();
        let body = serde_json::to_string(&game)?;
        let response = request_body!(self, patch, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed result about one specific game.](<https://developer.toornament.com/doc/games?#get:tournaments:tournament_id:matches:match_id:games:number:result>)
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
    pub fn match_game_result(
        &self,
        tournament_id: TournamentId,
        match_id: MatchId,
        game_number: GameNumber,
    ) -> Result<MatchResult> {
        log::debug!(
            "Getting match game result by tournament id and match id: {:?} / {:?}",
            tournament_id,
            match_id
        );
        let address = Endpoint::MatchGameResultGet {
            tournament_id,
            match_id,
            game_number,
        }
        .to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Updates or creates detailed result about one game.](<https://developer.toornament.com/doc/games?#put:tournaments:tournament_id:matches:match_id:games:number:result>)
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
    pub fn update_match_game_result(
        &self,
        tournament_id: TournamentId,
        match_id: MatchId,
        game_number: GameNumber,
        result: MatchResult,
        update_match: bool,
    ) -> Result<MatchResult> {
        log::debug!(
            "Setting match game result by tournament id and match id: {:?} / {:?}",
            tournament_id,
            match_id
        );
        let address = Endpoint::MatchGameResultUpdate {
            tournament_id,
            match_id,
            game_number,
            update_match,
        }
        .to_string();
        let body = serde_json::to_string(&result)?;
        let response = request_body!(self, put, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of participants from one tournament. The tournament must be public
    /// to have access to its participants, meaning the tournament organizer has published it. The
    /// participants are returned by 256.](<https://developer.toornament.com/doc/participant#get:tournaments:tournament_id:participants>)
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
    pub fn tournament_participants(
        &self,
        tournament_id: TournamentId,
        filter: TournamentParticipantsFilter,
    ) -> Result<Participants> {
        log::debug!(
            "Getting tournament participants by tournament id: {:?}",
            tournament_id
        );
        let address = Endpoint::Participants {
            tournament_id,
            filter,
        }
        .to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a participant in a tournament.](<https://developer.toornament.com/doc/participants?#post:tournaments:tournament_id:participants>)
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
    pub fn create_tournament_participant(
        &self,
        id: TournamentId,
        participant: Participant,
    ) -> Result<Participant> {
        log::debug!("Creating a participant for tournament with id: {:?}", id);
        let address = Endpoint::ParticipantCreate(id).to_string();
        let body = serde_json::to_string(&participant)?;
        let response = request_body!(self, post, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a list of participants in a tournament. If any participant already exists he will
    /// be erased.](<https://developer.toornament.com/doc/participants?_locale=en#put:tournaments:tournament_id:participants>)
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
    pub fn update_tournament_participants(
        &self,
        id: TournamentId,
        participants: Participants,
    ) -> Result<Participants> {
        log::debug!(
            "Creating a list of participants for tournament with id: {:?}",
            id
        );
        let address = Endpoint::ParticipantsUpdate(id).to_string();
        let body = serde_json::to_string(&participants)?;
        let response = request_body!(self, put, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns detailed information about one participant.](<https://developer.toornament.com/doc/participants?_locale=en#get:tournaments:tournament_id:participants:id>)
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
    pub fn tournament_participant(
        &self,
        id: TournamentId,
        participant_id: ParticipantId,
    ) -> Result<Participant> {
        log::debug!(
            "Getting tournament participant by tournament id and participant id: {:?} / {:?}",
            id,
            participant_id
        );
        let address = Endpoint::ParticipantById(id, participant_id).to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update some of the editable information on a participant.](<https://developer.toornament.com/doc/participants?_locale=en#patch:tournaments:tournament_id:participants:id>)
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
    /// participant = participant.name("Updated participant name here".to_owned());
    /// let updated_participant = t.update_tournament_participant(
    ///     TournamentId("1".to_owned()),
    ///     ParticipantId("2".to_owned()),
    ///     participant).unwrap();
    /// assert_eq!(updated_participant.id, Some(ParticipantId("2".to_owned())));
    /// assert_eq!(updated_participant.name, "Updated participant name here");
    /// ```
    pub fn update_tournament_participant(
        &self,
        id: TournamentId,
        participant_id: ParticipantId,
        participant: Participant,
    ) -> Result<Participant> {
        log::debug!(
            "Updating a participant for tournament with id and participant id: {:?} / {:?}",
            id,
            participant_id
        );
        let address = Endpoint::ParticipantById(id, participant_id).to_string();
        let body = serde_json::to_string(&participant)?;
        let response = request_body!(self, patch, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Deletes one participant.](<https://developer.toornament.com/doc/participants?_locale=en#delete:tournaments:tournament_id:participants:id>)
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
    pub fn delete_tournament_participant(
        &self,
        id: TournamentId,
        participant_id: ParticipantId,
    ) -> Result<()> {
        log::debug!(
            "Deleting a participant for tournament with id and participant id: {:?} / {:?}",
            id,
            participant_id
        );
        let address = Endpoint::ParticipantById(id, participant_id).to_string();
        let response = request!(self, delete, &address)?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Rest("Something went wrong"))
        }
    }

    /// [Returns a collection of permission from one tournament.](<https://developer.toornament.com/doc/permissions?_locale=en#get:tournaments:tournament_id:permissions>)
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
        log::debug!("Getting tournament permissions by tournament id: {:?}", id);
        let address = Endpoint::Permissions(id).to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Create a permission for a user on a tournament.](<https://developer.toornament.com/doc/permissions?_locale=en#post:tournaments:tournament_id:permissions>)
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
    pub fn create_tournament_permission(
        &self,
        id: TournamentId,
        permission: Permission,
    ) -> Result<Permission> {
        log::debug!("Creating tournament permissions by tournament id: {:?}", id);
        let address = Endpoint::Permissions(id).to_string();
        let body = serde_json::to_string(&permission)?;
        let response = request_body!(self, post, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Retrieves a permission of a tournament.](<https://developer.toornament.com/doc/permissions?_locale=en#get:tournaments:tournament_id:permissions:permission_id>)
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
    pub fn tournament_permission(
        &self,
        id: TournamentId,
        permission_id: PermissionId,
    ) -> Result<Permission> {
        log::debug!(
            "Getting tournament permission by tournament id and permission id: {:?} / {:?}",
            id,
            permission_id
        );
        let address = Endpoint::PermissionById(id, permission_id).to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Update rights of a permission.](<https://developer.toornament.com/doc/permissions?_locale=en#patch:tournaments:tournament_id:permissions:permission_id>)
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
    pub fn update_tournament_permission_attributes(
        &self,
        id: TournamentId,
        permission_id: PermissionId,
        attributes: PermissionAttributes,
    ) -> Result<Permission> {
        #[derive(serde::Serialize)]
        struct WrappedAttributes {
            attributes: PermissionAttributes,
        }
        log::debug!(
            "Updating tournament permission attributes by tournament id \
             and permission id: {:?} / {:?}",
            id,
            permission_id
        );
        let address = Endpoint::PermissionById(id, permission_id).to_string();
        let wrapped_attributes = WrappedAttributes { attributes };
        let body = serde_json::to_string(&wrapped_attributes)?;
        let response = request_body!(self, patch, &address, body)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Delete a user permission of a tournament.](<https://developer.toornament.com/doc/permissions?_locale=en#delete:tournaments:tournament_id:permissions:permission_id>)
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
    pub fn delete_tournament_permission(
        &self,
        id: TournamentId,
        permission_id: PermissionId,
    ) -> Result<()> {
        log::debug!(
            "Deleting a permission for tournament with id and permission id: {:?} / {:?}",
            id,
            permission_id
        );
        let address = Endpoint::PermissionById(id, permission_id).to_string();
        let response = request!(self, delete, &address)?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Rest("Something went wrong"))
        }
    }

    /// [Returns a collection of stages from one tournament. The tournament must be public to have
    /// access to its stages, meaning the tournament organizer must publish it.](<https://developer.toornament.com/doc/stages?_locale=en#get:tournaments:tournament_id:stages>)
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
        log::debug!("Getting tournament stages by tournament id: {:?}", id);
        let address = Endpoint::Stages(id).to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }

    /// [Returns a collection of videos from one tournament. The collection may be filtered and
    /// sorted by optional query parameters. The tournament must be public to have access to its
    /// videos, meaning the tournament organizer has published it. The videos are returned by 20.](<https://developer.toornament.com/doc/videos?_locale=en#get:tournaments:tournament_id:videos>)
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
    pub fn tournament_videos(
        &self,
        tournament_id: TournamentId,
        filter: TournamentVideosFilter,
    ) -> Result<Videos> {
        log::debug!(
            "Getting tournament videos by tournament id: {:?}",
            tournament_id
        );
        let address = Endpoint::Videos {
            tournament_id,
            filter,
        }
        .to_string();
        let response = request!(self, get, &address)?;

        Ok(serde_json::from_reader(response)?)
    }
}

#[cfg(test)]
mod tests {
    fn assert_sync_and_send<T: Sync + Send>() {}

    #[test]
    fn test_sync_and_send() {
        assert_sync_and_send::<crate::Toornament>();
    }
}
