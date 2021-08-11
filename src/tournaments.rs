use crate::common::Date;
use crate::disciplines::DisciplineId;
use crate::matches::{MatchFormat, MatchType};
use crate::participants::ParticipantType;
use crate::streams::Streams;

/// A tournament identity.
#[derive(
    Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct TournamentId(pub String);

/// A tournament status.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TournamentStatus {
    /// Implies the tournament has not started yet
    Setup,
    /// Means it has at least one match result
    Running,
    /// ??? No description in the source https://developer.toornament.com/doc/tournaments
    Pending,
    /// Indicates all matches have a result
    Completed,
}

/// A tournament object.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Tournament {
    /// An hexadecimal unique identifier for this tournament.
    /// Example: "5608fd12140ba061298b4569"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TournamentId>,
    /// This string is a unique identifier of a discipline.
    /// Example: "my_discipline"
    pub discipline: DisciplineId,
    /// Name of a tournament (maximum 30 characeters).
    /// Example: "My Weekly Tournament"
    pub name: String,
    /// Complete name of this tournament (maximum 80 characters).
    /// Example: "My Weekly Tournament - Long title"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Status of the tournament.
    /// Possible values: setup, running, completed
    pub status: TournamentStatus,
    /// Starting date of the tournament. This value uses the ISO 8601 date containing only the date section.
    /// Example: "2015-09-06"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_start: Option<Date>,
    /// Ending date of the tournament. This value uses the ISO 8601 date containing only the date section.
    /// Example: "2015-09-07"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_end: Option<Date>,
    /// Time zone of the tournament. This value is represented using the IANA tz database.
    /// Example: "America/Sao_Paulo"
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Whether the tournament is played on internet or not.
    /// Example: true
    pub online: bool,
    /// Whether the tournament is public or private.
    /// Example: true
    pub public: bool,
    /// Location (city, address, place of interest) of the tournament.
    /// Example: "London"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// Country of the tournament. This value uses the ISO 3166-1 alpha-2 country code.
    /// Example: "UK"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Size of a tournament. Represents the expected number of participants it'll be able to manage.
    /// Example: 16
    pub size: i64,
    /// Type of participants who plays in the tournament.
    /// Possible values: team, single
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_type: Option<ParticipantType>,
    /// Type of matches played in the tournament.
    /// Possible values: duel, ffa
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<MatchType>,
    /// Tournament organizer: individual, group, association or company.
    /// Example: "Avery Bullock"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// URL of the website
    /// Example: "http://www.toornament.com"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// User-defined description of the tournament (maximum 1,500 characters).
    /// Example: "My description \n on multiple lines"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User-defined rules of the tournament (maximum 10,000 characters).
    /// Example: "My rules \n on multiple lines"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    /// User-defined description of the tournament prizes (maximum 1,500 characters).
    /// Example: "1 - 10,000$ \n 2 - 5,000$"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize: Option<String>,
    /// (Optional) If the "participant type" value in this tournament is 'team', specify the smallest and the largest possible team sizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_size_min: Option<i64>,
    /// (Optional) If the "participant type" value in this tournament is 'team', specify the smallest and the largest possible team sizes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_size_max: Option<i64>,
    /// (Optional) A list of streams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streams: Option<Streams>,
    /// Enable or disable the participant check-in in the tournament.
    /// Example: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in: Option<bool>,
    /// Enable or disable the participant flag in the tournament.
    /// Example: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_nationality: Option<bool>,
    /// Define the default match format for every matches in the tournament.
    /// Possible values: none, one, home_away, bo3, bo5, bo7, bo9, bo11
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_format: Option<MatchFormat>,
}
impl Tournament {
    /// Creates new `Tournament` object.
    pub fn new<S: Into<String>>(
        id: Option<TournamentId>,
        discipline: DisciplineId,
        name: S,
        status: TournamentStatus,
        online: bool,
        public: bool,
        size: i64,
    ) -> Tournament {
        Tournament {
            id,
            discipline,
            name: name.into(),
            full_name: None,
            status,
            date_start: None,
            date_end: None,
            time_zone: None,
            online,
            public,
            location: None,
            country: None,
            size,
            participant_type: None,
            match_type: None,
            organization: None,
            website: None,
            description: None,
            rules: None,
            prize: None,
            team_size_min: None,
            team_size_max: None,
            streams: None,
            check_in: None,
            participant_nationality: None,
            match_format: None,
        }
    }

    /// A method which creates `Tournament` object for creation (Toornament::edit_tournament)
    /// purposes.
    pub fn create<S: Into<String>>(
        discipline: DisciplineId,
        name: S,
        size: i64,
        participant_type: ParticipantType,
    ) -> Tournament {
        Tournament {
            id: None,
            discipline,
            name: name.into(),
            full_name: None,
            status: TournamentStatus::Setup,
            date_start: None,
            date_end: None,
            time_zone: None,
            online: true,
            public: false,
            location: None,
            country: None,
            size,
            participant_type: Some(participant_type),
            match_type: None,
            organization: None,
            website: None,
            description: None,
            rules: None,
            prize: None,
            team_size_min: None,
            team_size_max: None,
            streams: None,
            check_in: None,
            participant_nationality: None,
            match_format: None,
        }
    }

    builder!(id, Option<TournamentId>);
    builder!(discipline, DisciplineId);
    builder_s!(name);
    builder_so!(full_name);
    builder!(status, TournamentStatus);
    builder!(date_start, Option<Date>);
    builder!(date_end, Option<Date>);
    builder_so!(time_zone);
    builder!(online, bool);
    builder!(public, bool);
    builder_so!(location);
    builder_so!(country);
    builder!(size, i64);
    builder!(participant_type, Option<ParticipantType>);
    builder!(match_type, Option<MatchType>);
    builder_so!(organization);
    builder_so!(website);
    builder_so!(description);
    builder_so!(rules);
    builder_so!(prize);
    builder!(team_size_min, Option<i64>);
    builder!(team_size_max, Option<i64>);
    builder!(streams, Option<Streams>);
    builder!(check_in, Option<bool>);
    builder!(participant_nationality, Option<bool>);
    builder!(match_format, Option<MatchFormat>);
}

impl Tournament {
    /// Returns iter for the tournament
    pub fn iter<'a>(&self, client: &'a crate::Toornament) -> Option<crate::TournamentIter<'a>> {
        self.id
            .clone()
            .map(|id| crate::TournamentIter::new(client, id).with_streams(self.streams.is_some()))
    }

    /// Converts tournament into an iter
    pub fn into_iter(self, client: &crate::Toornament) -> Option<crate::TournamentIter<'_>> {
        match self.id {
            Some(id) => {
                Some(crate::TournamentIter::new(client, id).with_streams(self.streams.is_some()))
            }
            None => None,
        }
    }
}

/// A list of `Tournament` objects.
#[derive(
    Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct Tournaments(pub Vec<Tournament>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_parse() {
        let string = r#"
        {
            "id": "56742bc7cc3c17ee608b4567",
            "name": "DreamhackCS",
            "url": "http://www.twitch.tv/dreamhackcs",
            "language": "en"
        }"#;
        let d: crate::Stream = serde_json::from_str(string).unwrap();

        assert_eq!(d.id.0, "56742bc7cc3c17ee608b4567");
        assert_eq!(d.name, "DreamhackCS");
        assert_eq!(d.url, "http://www.twitch.tv/dreamhackcs");
        assert_eq!(d.language, "en");
    }

    #[test]
    fn test_tournament_parse() {
        use chrono::Datelike;

        let string = r#"
        {
            "id": "5608fd12140ba061298b4569",
            "discipline": "my_discipline",
            "name": "My Weekly Tournament",
            "full_name": "My Weekly Tournament - Long title",
            "status": "running",
            "date_start": "2015-09-06",
            "date_end": "2015-09-07",
            "timezone": "America/Sao_Paulo",
            "online": true,
            "public": true,
            "location": "London",
            "country": "UK",
            "size": 16,
            "participant_type": "team",
            "match_type": "duel",
            "organization": "Avery Bullock",
            "website": "http://www.toornament.com",
            "description": "My description \n on multiple lines",
            "rules": "My rules \n on multiple lines",
            "prize": "1 - 10,000$ \n 2 - 5,000$",
            "streams": [
                {
                    "id": "56742bc7cc3c17ee608b4567",
                    "name": "DreamhackCS",
                    "url": "http://www.twitch.tv/dreamhackcs",
                    "language": "en"
                }
            ],
            "check_in": true,
            "participant_nationality": true,
            "match_format": "bo3"
        }"#;
        let t: Tournament = serde_json::from_str(string).unwrap();

        assert_eq!(t.id.clone().unwrap().0, "5608fd12140ba061298b4569");
        assert_eq!(t.discipline.0, "my_discipline");
        assert_eq!(t.name, "My Weekly Tournament");
        assert_eq!(t.status, TournamentStatus::Running);
        assert!(t.date_start.is_some());
        let date_start = t.date_start.clone().unwrap();
        assert_eq!(date_start.year(), 2015i32);
        assert_eq!(date_start.month(), 9u32);
        assert_eq!(date_start.day(), 6u32);
        assert!(t.date_end.is_some());
        let date_end = t.date_end.clone().unwrap();
        assert_eq!(date_end.year(), 2015i32);
        assert_eq!(date_end.month(), 9u32);
        assert_eq!(date_end.day(), 7u32);
        assert_eq!(t.time_zone, Some("America/Sao_Paulo".to_owned()));
        assert_eq!(t.online, true);
        assert_eq!(t.public, true);
        assert_eq!(t.location, Some("London".to_owned()));
        assert_eq!(t.country, Some("UK".to_owned()));
        assert_eq!(t.size, 16i64);
        assert_eq!(t.participant_type, Some(ParticipantType::Team));
        assert_eq!(t.match_type, Some(MatchType::Duel));
        assert_eq!(t.organization, Some("Avery Bullock".to_owned()));
        assert_eq!(t.website, Some("http://www.toornament.com".to_owned()));
        assert_eq!(
            t.description,
            Some("My description \n on multiple lines".to_owned())
        );
        assert_eq!(t.rules, Some("My rules \n on multiple lines".to_owned()));
        assert_eq!(t.prize, Some("1 - 10,000$ \n 2 - 5,000$".to_owned()));
        assert!(t.streams.is_some());
        let streams = t.streams.clone().unwrap();
        assert_eq!(streams.0.len(), 1);
        let stream_opt = streams.0.first();
        assert!(stream_opt.is_some());
        let stream = stream_opt.unwrap();
        assert_eq!(
            stream.id,
            crate::StreamId("56742bc7cc3c17ee608b4567".to_owned())
        );
        assert_eq!(stream.name, "DreamhackCS");
        assert_eq!(stream.url, "http://www.twitch.tv/dreamhackcs");
        assert_eq!(stream.language, "en");
        assert_eq!(t.check_in, Some(true));
        assert_eq!(t.participant_nationality, Some(true));
        assert_eq!(t.match_format, Some(MatchFormat::BestOf3));
    }
}
