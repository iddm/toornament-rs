use ::*;

/// Disciplines iterator
pub struct DisciplinesIter<'a> {
    client: &'a Toornament,

    all: bool,

    disciplines_iter: Option<<Disciplines as IntoIterator>::IntoIter>,
    fetched: bool,
}
impl<'a> DisciplinesIter<'a> {
    /// Creates new disciplines iterator
    pub fn new(client: &'a Toornament) -> DisciplinesIter<'a> {
        DisciplinesIter {
            client: client,
            all: true,
            disciplines_iter: None,
            fetched: false,
        }
    }

    /// Marks for fetching all the disciplines
    pub fn all(mut self) -> Self {
        self.all = true;
        self
    }

    /// Fetches the disciplines
    fn fetch(&mut self) {
        if self.fetched {
            return;
        }

        self.fetched = true;

        let disciplines = match self.all {
            _ => self.client.disciplines(None),
        };

        self.disciplines_iter = match disciplines {
            Ok(d) => Some(d.into_iter()),
            Err(e) => {
                error!("Could not fetch disciplines during iteration: {:?}", e);
                None
            }
        };
    }

    /// Refetch the disciplines
    pub fn refetch(&mut self) {
        self.fetched = false;
        self.fetch();
    }
}

/// Iterator implementation
impl<'a> Iterator for DisciplinesIter<'a> {
    type Item = Discipline;

    fn next(&mut self) -> Option<Self::Item> {
        self.fetch();

        match self.disciplines_iter {
            Some(ref mut iter) => iter.next(),
            None => None,
        }
    }
}

/// Modifiers
impl<'a> DisciplinesIter<'a> {
    /// Fetch a discipline with id
    pub fn with_id(self, id: DisciplineId) -> DisciplineIter<'a> {
        DisciplineIter {
            client: self.client,
            id: id,
        }
    }
}

/// Discipline iterator
pub struct DisciplineIter<'a> {
    client: &'a Toornament,

    /// Fetch a discipline with the following id
    id: DisciplineId,
}

impl<'a> DisciplineIter<'a> {
    /// Creates new discipline iterator
    pub fn new(client: &'a Toornament, id: DisciplineId) -> DisciplineIter<'a> {
        DisciplineIter {
            client: client,
            id: id,
        }
    }
}

/// Modifiers
impl<'a> DisciplineIter<'a> {
    /// Fetch matches of a discipline
    pub fn matches(self) -> DisciplineMatchesIter<'a> {
        DisciplineMatchesIter::new(self.client, self.id)
    }
}

/// Terminators
impl<'a> DisciplineIter<'a> {
    /// Fetch the discipline
    pub fn collect<T: From<Discipline>>(self) -> Result<T> {
        /// Option::take() returns reference? wtf?
        match self.client.disciplines(Some(self.id.clone()))?.0.first().take() {
            Some(d) => Ok(T::from(d.to_owned())),
            None => Err(Error::Iter(IterError::NoSuchDiscipline(self.id))),
        }
    }
}
