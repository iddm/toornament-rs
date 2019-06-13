use *;

/// Tournament videos iterator
pub struct VideosIter<'a> {
    client: &'a Toornament,

    /// Fetch videos of the following tournament id
    tournament_id: TournamentId,
    /// Fetch filter
    filter: TournamentVideosFilter,
}
impl<'a> VideosIter<'a> {
    /// Create new videos iter
    pub fn new(client: &'a Toornament, tournament_id: TournamentId) -> VideosIter {
        VideosIter {
            client: client,
            tournament_id: tournament_id,
            filter: TournamentVideosFilter::default(),
        }
    }
}

/// Builders
impl<'a> VideosIter<'a> {
    /// Filter videos
    pub fn with_filter(mut self, filter: TournamentVideosFilter) -> Self {
        self.filter = filter;
        self
    }
}

/// Terminators
impl<'a> VideosIter<'a> {
    /// Collect the videos
    pub fn collect<T: From<Videos>>(self) -> Result<T> {
        Ok(T::from(
            self.client
                .tournament_videos(self.tournament_id, self.filter)?,
        ))
    }
}
