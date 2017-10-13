use ::*;
use iter::participants::ParticipantsIter;
use iter::permissions::PermissionsIter;
use iter::tournament_matches::TournamentMatchesIter;
use iter::stages::StagesIter;
use iter::videos::VideosIter;
use std::iter::Iterator;


#[derive(Debug, Copy, Clone)]
enum TournamentsIterFetch {
    All,
    My,
}

/// A remote iterator over tournaments
#[derive(Debug)]
pub struct TournamentsIter<'a> {
    client: &'a Toornament,

    /// Fetch tournaments with the streams
    with_streams: bool,
    /// Fetch tournaments with the following name
    name: Option<String>,
    /// Fetch type
    fetch: TournamentsIterFetch,
}
impl<'a> TournamentsIter<'a> {
    /// Creates new tournaments iterator
    pub fn new(client: &'a Toornament) -> TournamentsIter {
        TournamentsIter {
            client: client,
            with_streams: false,
            name: None,
            fetch: TournamentsIterFetch::All,
            // ..Default::default()
        }
    }
}
impl<'a> Iterator for TournamentsIter<'a> {
    type Item = Tournament;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

/// Builders
impl<'a> TournamentsIter<'a> {
    /// Fetch a tournament with the following name
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Fetch all my tournaments
    pub fn my(mut self) -> Self {
        self.fetch = TournamentsIterFetch::My;
        self
    }

    /// Fetch all the tournaments
    pub fn all(mut self) -> Self {
        self.fetch = TournamentsIterFetch::All;
        self
    }

    /// Fetch with the streams
    pub fn with_streams(mut self, with_streams: bool) -> Self {
        self.with_streams = with_streams;
        self
    }
}

/// Modifiers
impl<'a> TournamentsIter<'a> {
    /// Fetch a tournament with the following id
    pub fn with_id(self, id: TournamentId) -> TournamentIter<'a> {
        TournamentIter::new(self.client, id).with_streams(self.with_streams)
    }

    /// Create a tournament
    pub fn create<F: 'static + FnMut() -> Tournament>(self, creator: F)
        -> TournamentCreator<'a> {
        TournamentCreator {
            client: self.client,
            creator: Box::new(creator),
        }
    }
}

/// Terminators
impl<'a> TournamentsIter<'a> {
    /// Return the collection
    pub fn collect<T: From<Tournaments>>(self) -> Result<T> {
        let mut tournaments = match self.fetch {
            TournamentsIterFetch::All => {
                self.client.tournaments(None, self.with_streams)
            },
            TournamentsIterFetch::My => {
                self.client.my_tournaments()
            },
        }?;

        if let Some(name) = self.name {
            tournaments.0.retain(|t| t.name == name);
        }

        Ok(T::from(tournaments))
    }
}


/// A remote tournament iterator
pub struct TournamentIter<'a> {
    client: &'a Toornament,

    /// A tournament id
    id: TournamentId,
    /// Should include streams
    with_streams: bool,
}
impl<'a> TournamentIter<'a> {
    /// Creates new tournament iter for a tournament with id
    pub fn new(client: &'a Toornament, id: TournamentId) -> TournamentIter {
        TournamentIter {
            client: client,
            id: id,
            with_streams: false,
        }
    }
}

/// Builders
impl<'a> TournamentIter<'a> {
    /// Fetch streams
    pub fn with_streams(mut self, with_streams: bool) -> Self {
        self.with_streams = with_streams;
        self
    }

    /// Set tournament id
    pub fn with_id(mut self, id: TournamentId) -> Self {
        self.id = id;
        self
    }
}

/// Modifiers
impl<'a> TournamentIter<'a> {
    /// Tournament lazy editor
    pub fn edit<F: 'static + FnMut(Tournament) -> Tournament>(self, editor: F)
        -> TournamentEditor<'a> {
        TournamentEditor {
            client: self.client,
            id: self.id,
            with_streams: self.with_streams,
            editor: Box::new(editor),
        }
    }

    /// Tournament participants
    pub fn participants(self) -> ParticipantsIter<'a> {
        ParticipantsIter::new(self.client, self.id)
    }

    /// Tournament matches
    pub fn matches(self) -> TournamentMatchesIter<'a> {
        TournamentMatchesIter::new(self.client, self.id)
    }

    /// Tournament permissions
    pub fn permissions(self) -> PermissionsIter<'a> {
        PermissionsIter::new(self.client, self.id)
    }

    /// Tournament stages
    pub fn stages(self) -> StagesIter<'a> {
        StagesIter::new(self.client, self.id)
    }

    /// Tournament videos
    pub fn videos(self) -> VideosIter<'a> {
        VideosIter::new(self.client, self.id)
    }
}

/// Terminators
impl<'a> TournamentIter<'a> {
    /// Return the tournament
    pub fn collect<T: From<Tournament>>(self) -> Result<T> {
        let tournaments = self.client.tournaments(Some(self.id.clone()), self.with_streams)?;
        let tournament = match tournaments.0.first() {
            Some(t) => t.to_owned(),
            None => return Err(Error::Iter(IterError::NoSuchTournament(self.id))),
        };

        Ok(T::from(tournament))
    }

    /// Deletes the tournament
    pub fn delete(self) -> Result<()> {
        self.client.delete_tournament(self.id)
    }
}

/// A lazy tournament editor
pub struct TournamentEditor<'a> {
    client: &'a Toornament,

    /// Tournament id
    id: TournamentId,
    /// With streams
    with_streams: bool,
    /// Tournament editor
    editor: Box<FnMut(Tournament) -> Tournament>,
}

/// Terminators
impl<'a> TournamentEditor<'a> {
    /// Sends the edited tournament
    pub fn update(mut self) -> Result<Tournament> {
        let tournaments = self.client.tournaments(Some(self.id.clone()), self.with_streams)?;
        let original = match tournaments.0.first() {
            Some(t) => t.to_owned(),
            None => return Err(Error::Iter(IterError::NoSuchTournament(self.id))),
        };
        let edited = (self.editor)(original);
        self.client.edit_tournament(edited)
    }

    /// Update and return iter
    pub fn update_iter(mut self) -> Result<TournamentIter<'a>> {
        let tournaments = self.client.tournaments(Some(self.id.clone()), self.with_streams)?;
        let original = match tournaments.0.first() {
            Some(t) => t.to_owned(),
            None => return Err(Error::Iter(IterError::NoSuchTournament(self.id))),
        };
        let edited = (self.editor)(original);
        let _ = self.client.edit_tournament(edited)?;
        Ok(TournamentIter {
            client: self.client,
            id: self.id,
            with_streams: self.with_streams,
        })
    }
}

/// A lazy tournament creator
pub struct TournamentCreator<'a> {
    client: &'a Toornament,

    /// Tournament creator
    creator: Box<FnMut() -> Tournament>,
}

/// Terminators
impl<'a> TournamentCreator<'a> {
    /// Creates the tournament
    pub fn update(mut self) -> Result<Tournament> {
        self.client.edit_tournament((self.creator)())
    }

    /// Create and return iter
    pub fn update_iter(mut self) -> Result<TournamentIter<'a>> {
        let created = self.client.edit_tournament((self.creator)())?;

        match created.id {
            Some(id) => Ok(TournamentIter::new(self.client, id)),
            None => Err(Error::Iter(IterError::NoTournamentId(created))),
        }
    }
}
