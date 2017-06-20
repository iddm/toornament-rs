use ::*;

/// Disciplines iterator
pub struct DisciplinesIter<'a> {
    client: &'a Toornament,

    all: bool,
}
impl<'a> DisciplinesIter<'a> {
    /// Creates new disciplines iterator
    pub fn new(client: &'a Toornament) -> DisciplinesIter<'a> {
        DisciplinesIter {
            client: client,
            all: true,
        }
    }

    /// Fetch all disciplines
    pub fn all(mut self) -> Self {
        self.all = true;
        self
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

/// Terminators
impl<'a> DisciplinesIter<'a> {
    /// Fetch the discipline
    pub fn collect<T: From<Disciplines>>(self) -> Result<T> {
        match self.all {
            _ => Ok(T::from(self.client.disciplines(None)?)),
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
