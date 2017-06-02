use matches::MatchType;
use common::Date;
use disciplines::DisciplineId;
use participants::ParticipantType;


/// A tournament identity.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TournamentId(pub String);

/// A tournament status.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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

/// A stream identity.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct StreamId(pub String);

/// A stream object.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Stream {
    /// An hexadecimal unique identifier for this stream.
    /// Example: "56742bc7cc3c17ee608b4567"
    pub id: StreamId,
    /// Title of the stream.
    /// Example: "DreamhackCS"
    pub name: String,
    /// Url of the stream.
    /// Example: "http://www.twitch.tv/dreamhackcs"
    pub url: String,
    /// Language code of the stream content. This value is represented as an ISO 639-1 code.
    /// Example: "en"
    pub language: String,
}

/// A list of `Stream` objects.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Streams(pub Vec<Stream>);

/// A Match format enumeration.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MatchFormat {
    /// Needs description
    #[serde(rename = "none")]
    None,
    /// Needs description
    #[serde(rename = "one")]
    One,
    /// Needs description
    #[serde(rename = "home_away")]
    HomeAway,
    /// Best of 3
    #[serde(rename = "bo3")]
    BestOf3,
    /// Best of 5
    #[serde(rename = "bo5")]
    BestOf5,
    /// Best of 7
    #[serde(rename = "bo7")]
    BestOf7,
    /// Best of 9
    #[serde(rename = "bo9")]
    BestOf9,
    /// Best of 11
    #[serde(rename = "bo11")]
    BestOf11,
}

/// A tournament object.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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
    /// Creates new `Discipline` object.
    pub fn new<S: Into<String>>(id: Option<TournamentId>,
                                discipline: DisciplineId,
                                name: S,
                                status: TournamentStatus,
                                online: bool,
                                public: bool,
                                size: i64) -> Tournament {
        Tournament {
            id: id,
            discipline: discipline,
            name: name.into(),
            full_name: None,
            status: status,
            date_start: None,
            date_end: None,
            time_zone: None,
            online: online,
            public: public,
            location: None,
            country: None,
            size: size,
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

/// A list of `Tournament` objects.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Tournaments(pub Vec<Tournament>);


#[cfg(test)]
mod tests {
    use ::serde_json;

    #[test]
    fn test_stream_parse() {
        use tournaments::Stream;
        let string = r#"
        {
            "id": "56742bc7cc3c17ee608b4567",
            "name": "DreamhackCS",
            "url": "http://www.twitch.tv/dreamhackcs",
            "language": "en"
        }"#;
        let d: Stream = serde_json::from_str(string).unwrap();

        assert_eq!(d.id.0, "56742bc7cc3c17ee608b4567");
        assert_eq!(d.name, "DreamhackCS");
        assert_eq!(d.url, "http://www.twitch.tv/dreamhackcs");
        assert_eq!(d.language, "en");
    }

    #[test]
    fn test_tournament_parse() {
        use matches::MatchType;
        use tournaments::{ ParticipantType,
            MatchFormat,
            Tournament,
            TournamentStatus,
            StreamId,
        };
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
        assert_eq!(t.description, Some("My description \n on multiple lines".to_owned()));
        assert_eq!(t.rules, Some("My rules \n on multiple lines".to_owned()));
        assert_eq!(t.prize, Some("1 - 10,000$ \n 2 - 5,000$".to_owned()));
        assert!(t.streams.is_some());
        let streams = t.streams.clone().unwrap();
        assert_eq!(streams.0.len(), 1);
        let stream_opt = streams.0.first();
        assert!(stream_opt.is_some());
        let stream = stream_opt.unwrap();
        assert_eq!(stream.id, StreamId("56742bc7cc3c17ee608b4567".to_owned()));
        assert_eq!(stream.name, "DreamhackCS");
        assert_eq!(stream.url, "http://www.twitch.tv/dreamhackcs");
        assert_eq!(stream.language, "en");
        assert_eq!(t.check_in, Some(true));
        assert_eq!(t.participant_nationality, Some(true));
        assert_eq!(t.match_format, Some(MatchFormat::BestOf3));
    }
}
