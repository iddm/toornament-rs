/// Unique participant identifier
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ParticipantId(pub String);

/// A participant type enumeration.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParticipantType {
    /// Implies the tournament is played by teams
    Team,
    /// Means the tournament is played by players
    Single,
}

/// An opponent involved in a match.
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Participant {
    /// Unique identifier for this participant.
    pub id: ParticipantId,
    /// Participant name (maximum 40 characters).
    pub name: String,
    /// Country of the participant. This property is only available when the "country"
    /// option is enabled for this tournament. This value is represented as an ISO 3166-1
    /// alpha-2 country code.
    pub country: Option<String>,
}
