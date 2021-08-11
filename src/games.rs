use crate::matches::MatchStatus;
use crate::opponents::Opponents;

/// A game number.
#[derive(
    Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct GameNumber(pub i64);

/// A game description.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Game {
    /// Game's number.
    pub number: GameNumber,
    /// Game's status
    pub status: MatchStatus,
    /// Game's opponents
    pub opponents: Opponents,
}

/// Array of games
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Games(pub Vec<Game>);
