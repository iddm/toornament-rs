use matches::MatchStatus;
use common::Opponents;

/// A game number.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GameNumber(pub i64);

/// A game description.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Game {
    /// Game's number.
    pub number: GameNumber,
    /// Game's status
    pub status: MatchStatus,
    /// Game's opponents
    pub opponents: Opponents,
}

/// Array of games
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Games(pub Vec<Game>);
