use ::*;


/// Tournament stages iterator
pub struct StagesIter<'a> {
    client: &'a Toornament,

    /// Fetch stages of the following tournament id
    tournament_id: TournamentId,
}
impl<'a> StagesIter<'a> {
    /// Create new stages iter
    pub fn new(client: &'a Toornament, tournament_id: TournamentId) -> StagesIter {
        StagesIter {
            client: client,
            tournament_id: tournament_id,
        }
    }
}

/// Terminators
impl<'a> StagesIter<'a> {
    /// Collect the stages
    pub fn collect<T: From<Stages>>(self) -> Result<T> {
        Ok(T::from(self.client.tournament_stages(self.tournament_id)?))
    }
}
