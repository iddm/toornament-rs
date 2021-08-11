use crate::common::MatchResultSimple;
use crate::participants::Participant;

/// An opponent involved in a match.
#[derive(
    Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct Opponent {
    /// Number of the opponent
    pub number: i64,
    /// The participant represented in this opponent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant: Option<Participant>,
    /// The result of the opponent. This property is only available on "duel" match format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<MatchResultSimple>,
    /// Rank of the opponent, compared to other opponents' ranks.
    /// This property is only available on matches of type "ffa".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// The score of this game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
    /// Whether the opponent has forfeited or not.
    pub forfeit: bool,
}

/// List of the opponents involved in this match.
#[derive(
    Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct Opponents(pub Vec<Opponent>);
