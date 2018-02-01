//! This module introduces iterator-like interface to the toornament.
//! The content of this module is not really an iterator, it just may look so.
//! It was made to provide an easy and rust-idiomatic way to interact with the service.
//! Note that everything here is "lazy". Think of it as you use an iterator over remote data.
//!
//! # Usage
//!
//! Delete a participant:
//!
//! ```rust,no_run
//! use toornament::*;
//!
//! let toornament = Toornament::with_application("API_TOKEN",
//!                                               "CLIENT_ID",
//!                                               "CLIENT_SECRET").unwrap();
//! toornament.tournaments_iter()
//!           .with_id(TournamentId("1".to_owned()))
//!           .participants()
//!           .with_id(ParticipantId("2".to_owned()))
//!           .delete();
//! ```
//!
//! Iterate over tournaments and some other actions:
//!
//! ```rust,no_run
//! use toornament::*;
//!
//! let toornament = Toornament::with_application("API_TOKEN",
//!                                               "CLIENT_ID",
//!                                               "CLIENT_SECRET").unwrap();
//!
//! let all = toornament.tournaments_iter().all().collect::<Tournaments>().unwrap();
//! let my = toornament.tournaments_iter().my().collect::<Tournaments>().unwrap();
//! let tournament_with_id_1 = toornament.tournaments_iter()
//!                                      .with_id(TournamentId("1".to_owned()))
//!                                      .collect::<Tournament>().unwrap();
//! // Edit tournament with id = 1
//! let edit_result = toornament.tournaments_iter()
//!                             .with_id(TournamentId("1".to_owned()))
//!                             .edit(|t| t.name("New name"))
//!                             .update();
//! // Delete tournament with id = 1
//! let delete_result = toornament.tournaments_iter()
//!                               .with_id(TournamentId("1".to_owned()))
//!                               .delete();
//! // Get tournament's permissions
//! let permissions = toornament.tournaments_iter()
//!                             .with_id(TournamentId("1".to_owned()))
//!                             .permissions()
//!                             .collect::<Permissions>();
//! // Get tournament's participants
//! let participants = toornament.tournaments_iter()
//!                              .with_id(TournamentId("1".to_owned()))
//!                              .participants()
//!                              .collect::<Participants>();
//! // Edit tournament's match
//! let edited_match = toornament.tournaments_iter()
//!                              .with_id(TournamentId("1".to_owned()))
//!                              .matches()
//!                              .with_id(MatchId("2".to_owned()))
//!                              .edit(|m| m.number(2))
//!                              .update();
//! // Get match result
//! let match_result = toornament.tournaments_iter()
//!                              .with_id(TournamentId("1".to_owned()))
//!                              .matches()
//!                              .with_id(MatchId("2".to_owned()))
//!                              .result()
//!                              .collect::<MatchResult>();
//! // Get match game result
//! let game_result = toornament.tournaments_iter()
//!                             .with_id(TournamentId("1".to_owned()))
//!                             .matches()
//!                             .with_id(MatchId("2".to_owned()))
//!                             .games()
//!                             .with_number(GameNumber(3i64))
//!                             .result()
//!                             .collect::<MatchResult>();
//! ```
//!
//! Note that iter-like interface is lazy - no action is done before you actually do something.
//! So, the finish states are usually a modifier of an iterator (like `matches()` of
//! `TournamentIter`) or a `collect()` methods.

mod tournaments;
mod tournament_matches;
mod games;
mod participants;
mod permissions;
mod stages;
mod videos;
mod disciplines;
mod discipline_matches;

pub use self::tournaments::*;
pub use self::tournament_matches::*;
pub use self::games::*;
pub use self::participants::*;
pub use self::permissions::*;
pub use self::stages::*;
pub use self::videos::*;
pub use self::disciplines::*;
pub use self::discipline_matches::*;
