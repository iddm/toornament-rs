use matches::MatchId;

use std::fmt;

/// Tournament video category
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VideoCategory {
    /// Replay video
    Replay,
    /// Highlight video
    Highlight,
    /// Bonus video
    Bonus,
}
impl fmt::Display for VideoCategory {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VideoCategory::Replay => fmt.write_str("replay"),
            VideoCategory::Highlight => fmt.write_str("hightlight"),
            VideoCategory::Bonus => fmt.write_str("bonus"),
        }
    }
}

/// A tournament video
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Video {
    /// Title of the video.
    pub name: String,
    /// Url of the video.
    pub url: String,
    /// Language code of the video content. This value is represented as an ISO 639-1 code.
    pub language: String,
    /// Category of the video.
    pub category: VideoCategory,
    /// The match's unique identifier of this video.
    pub match_id: Option<MatchId>,
}

/// A list of tournament videos
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Videos(pub Vec<Video>);


#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_videos_parse() {
        let string = r#"
        [
            {
                "name": "Game 1: TSM vs. EnVyUs",
                "url": "https://www.youtube.com/watch?v=SI5QgDJkaSU",
                "language": "en",
                "category": "replay",
                "match_id": "5617bb3af3df95f2318b4567"
            }
        ]
        "#;

        let videos: Videos = serde_json::from_str(string).unwrap();

        assert_eq!(videos.0.len(), 1);
        let v = videos.0.first().unwrap().clone();
        assert_eq!(v.name, "Game 1: TSM vs. EnVyUs");
        assert_eq!(v.url, "https://www.youtube.com/watch?v=SI5QgDJkaSU");
        assert_eq!(v.language, "en");
        assert_eq!(v.match_id, Some(MatchId("5617bb3af3df95f2318b4567".to_owned())));
    }
}
