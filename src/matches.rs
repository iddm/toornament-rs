use chrono::{ DateTime, FixedOffset };

use disciplines::DisciplineId;
use tournaments::TournamentId;
use common::Opponents;
use games::Games;

/// Match unique identificator.
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MatchId(pub String);

/// A match type enumeration.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MatchType {
    /// Duel match type
    #[serde(rename = "duel")]
    Duel,
    /// FFA match type
    #[serde(rename = "ffa")]
    FreeForAll,
}

/// A match status.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MatchStatus {
    /// Implies the match has not started yet
    Pending,
    /// Means it has started but not yet ended
    Running,
    /// Indicates the match is finished
    Completed
}

/// Tournament or discipline match definition.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Match {
    /// A hexadecimal unique identifier for this match.
    /// Example: "5617bb3af3df95f2318b4567"
    pub id: MatchId,
    /// Type of match: "duel" means only two opponents are involved; "ffa" means more than two opponents are involved.
    /// Possible values: duel, ffa
    #[serde(rename = "type")]
    pub match_type: MatchType,
    /// The discipline unique identifier of the match.
    /// Example: "my_discipline"
    #[serde(rename = "discipline")]
    pub discipline_id: DisciplineId,
    /// Status of the match: "pending" implies it has not yet started; "running" means it has started but not yet ended; "completed" indicates the match is finished.
    /// Possible values: pending, running, completed
    pub status: MatchStatus,
    /// The tournament's unique identifier of this match.
    /// Example: "5608fd12140ba061298b4569"
    pub tournament_id: TournamentId,
    /// Number of this match.
    /// Example: 1
    pub number: u64,
    /// Stage number of this match.
    /// Example: 1
    pub stage_number: u64,
    /// Group number of this match.
    /// Example: 1
    pub group_number: u64,
    /// Round number of this match.
    /// Example: 1
    pub round_number: u64,
    /// Date of this match, either expected or actual. This value is represented as an ISO 8601 date containing the date, the time and the time zone.
    /// Example: "2015-09-06T00:10:00-0600"
    pub date: DateTime<FixedOffset>,
    /// List of the opponents involved in this match.
    pub opponents: Opponents,
    /// This property is added when the parameter "with_games" is enabled.
    pub games: Option<Games>,
}
impl Match {
    builder!(id, MatchId);
    builder!(match_type, MatchType);
    builder!(discipline_id, DisciplineId);
    builder!(status, MatchStatus);
    builder!(tournament_id, TournamentId);
    builder!(number, u64);
    builder!(stage_number, u64);
    builder!(group_number, u64);
    builder!(round_number, u64);
    builder!(date, DateTime<FixedOffset>);
}

/// A list of `Match` objects.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Matches(pub Vec<Match>);

/// Result of a match
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MatchResult {
    /// Status of a match
    pub status: MatchStatus,
    /// Opponents in a match
    pub opponents: Opponents,
}


#[cfg(test)]
mod tests {
    use ::serde_json;

    #[test]
    fn test_match_parse() {
        use matches::{ Match, MatchType, MatchStatus };
        let string = r#"
        {
            "id": "5617bb3af3df95f2318b4567",
            "type": "duel",
            "discipline": "my_discipline",
            "status": "pending",
            "tournament_id": "5608fd12140ba061298b4569",
            "number": 1,
            "stage_number": 1,
            "group_number": 2,
            "round_number": 3,
            "date": "2015-09-06T00:10:00-0600",
            "timezone": "America\/Chicago",
            "match_format": "bo3",
            "opponents": [
                {
                    "number": 1,
                    "participant": {
                        "id": "5617c3acf3df959e368b4567",
                        "name": "Evil Geniuses",
                        "country": "US"
                    },
                    "result": 1,
                    "score": null,
                    "forfeit": false
                }
            ]
        }"#;
        let d: Match = serde_json::from_str(string).unwrap();

        assert_eq!(d.id.0, "5617bb3af3df95f2318b4567");
        assert_eq!(d.match_type, MatchType::Duel);
        assert_eq!(d.discipline_id.0, "my_discipline");
        assert_eq!(d.status, MatchStatus::Pending);
        assert_eq!(d.tournament_id.0, "5608fd12140ba061298b4569");
        assert_eq!(d.number, 1u64);
        assert_eq!(d.stage_number, 1u64);
        assert_eq!(d.group_number, 2u64);
        assert_eq!(d.round_number, 3u64);
    }

    #[test]
    fn test_parse_match_results() {
        use matches::{ MatchStatus, MatchResult };
        use common::MatchResultSimple;
        let string = r#"
        {
            "status": "pending",
            "opponents": [
                {
                    "number": 1,
                    "result": 1,
                    "score": null,
                    "forfeit": false
                }
            ]
        }"#;
        let r: MatchResult = serde_json::from_str(string).unwrap();

        assert_eq!(r.status, MatchStatus::Pending);
        let op = r.opponents.0.iter().next().unwrap();
        assert_eq!(op.number, 1);
        assert_eq!(op.result, Some(MatchResultSimple::Win));
        assert_eq!(op.score, None);
        assert_eq!(op.forfeit, false);
    }
}
