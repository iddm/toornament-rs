use *;

/// A discipline matches iterator
pub struct DisciplineMatchesIter<'a> {
    client: &'a Toornament,

    /// Fetch matches of discipline
    discipline_id: DisciplineId,
    /// Fetch match with filter
    filter: MatchFilter,
}
impl<'a> DisciplineMatchesIter<'a> {
    /// Creates new match iterator
    pub fn new(client: &'a Toornament, id: DisciplineId) -> DisciplineMatchesIter {
        DisciplineMatchesIter {
            client: client,
            discipline_id: id,
            filter: MatchFilter::default(),
        }
    }
}

/// Builders
impl<'a> DisciplineMatchesIter<'a> {
    /// Fetch matches with filter
    pub fn with_filter(mut self, filter: MatchFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Fetch match by discipline id
    pub fn of_discipline(mut self, id: DisciplineId) -> Self {
        self.discipline_id = id;
        self
    }
}

/// Terminators
impl<'a> DisciplineMatchesIter<'a> {
    /// Fetch matches
    pub fn collect<T: From<Matches>>(self) -> Result<T> {
        Ok(T::from(
            self.client
                .matches_by_discipline(self.discipline_id, self.filter)?,
        ))
    }
}
