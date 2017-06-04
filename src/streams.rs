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
