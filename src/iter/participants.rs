use *;

/// A remote participants iterator
pub struct ParticipantsIter<'a> {
    client: &'a Toornament,

    /// Participants of the following tournament id
    tournament_id: TournamentId,
    /// Participants with filter
    filter: TournamentParticipantsFilter,
}
impl<'a> ParticipantsIter<'a> {
    /// Create new participants iter
    pub fn new(client: &'a Toornament, tournament_id: TournamentId) -> ParticipantsIter {
        ParticipantsIter {
            client,
            tournament_id,
            filter: TournamentParticipantsFilter::default(),
        }
    }
}

/// Builders
impl<'a> ParticipantsIter<'a> {
    /// Filter participants
    pub fn with_filter(mut self, filter: TournamentParticipantsFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Fetch participant of tournament with id
    pub fn of_tournament(mut self, id: TournamentId) -> Self {
        self.tournament_id = id;
        self
    }
}

/// Modifiers
impl<'a> ParticipantsIter<'a> {
    /// Fetch participant with id
    pub fn with_id(self, id: ParticipantId) -> ParticipantIter<'a> {
        ParticipantIter::new(self.client, self.tournament_id, id)
    }

    /// Update the list of participants
    pub fn edit<F: 'static + FnMut(Participants) -> Participants>(
        self,
        editor: F,
    ) -> ParticipantsEditor<'a> {
        ParticipantsEditor {
            client: self.client,
            tournament_id: self.tournament_id,
            filter: self.filter,
            editor: Box::new(editor),
        }
    }

    /// Create a participant
    pub fn create<F: 'static + FnMut() -> Participant>(self, creator: F) -> ParticipantCreator<'a> {
        ParticipantCreator {
            client: self.client,
            tournament_id: self.tournament_id,
            creator: Box::new(creator),
        }
    }
}

/// Terminators
impl<'a> ParticipantsIter<'a> {
    /// Collects the participants
    pub fn collect<T: From<Participants>>(self) -> Result<T> {
        Ok(T::from(self.client.tournament_participants(
            self.tournament_id,
            self.filter,
        )?))
    }
}

/// A lazy participants editor
pub struct ParticipantsEditor<'a> {
    client: &'a Toornament,

    /// Tournament id in which the participants is in
    tournament_id: TournamentId,
    /// Participants with filter
    filter: TournamentParticipantsFilter,
    /// Participant editor
    editor: Box<dyn FnMut(Participants) -> Participants>,
}

/// Terminators
impl<'a> ParticipantsEditor<'a> {
    /// Sends the edited participant
    pub fn update(mut self) -> Result<Participants> {
        let original = self
            .client
            .tournament_participants(self.tournament_id.clone(), self.filter)?;
        let edited = (self.editor)(original);
        self.client
            .update_tournament_participants(self.tournament_id, edited)
    }
}

/// A remote participant iterator
pub struct ParticipantIter<'a> {
    client: &'a Toornament,

    /// Fetch a participant with the following id
    tournament_id: TournamentId,
    /// Fetch a participant with the following id
    id: ParticipantId,
}
impl<'a> ParticipantIter<'a> {
    /// Create new participant iter
    pub fn new(
        client: &'a Toornament,
        tournament_id: TournamentId,
        id: ParticipantId,
    ) -> ParticipantIter {
        ParticipantIter {
            client,
            tournament_id,
            id,
        }
    }
}

/// Modifiers
impl<'a> ParticipantIter<'a> {
    /// Edit the participant
    pub fn edit<F: 'static + FnMut(Participant) -> Participant>(
        self,
        editor: F,
    ) -> ParticipantEditor<'a> {
        ParticipantEditor {
            client: self.client,
            tournament_id: self.tournament_id,
            id: self.id,
            editor: Box::new(editor),
        }
    }
}

/// Terminators
impl<'a> ParticipantIter<'a> {
    /// Collects the participant
    pub fn collect<T: From<Participant>>(self) -> Result<T> {
        Ok(T::from(
            self.client
                .tournament_participant(self.tournament_id, self.id)?,
        ))
    }

    /// Delete the participant
    pub fn delete(self) -> Result<()> {
        self.client
            .delete_tournament_participant(self.tournament_id, self.id)
    }

    /// Update the participant
    pub fn update(self, participant: Participant) -> Result<Participant> {
        self.client
            .update_tournament_participant(self.tournament_id, self.id, participant)
    }
}

/// A lazy participant creator
pub struct ParticipantCreator<'a> {
    client: &'a Toornament,

    /// Tournament id in which the participant is in
    tournament_id: TournamentId,
    /// Participant editor
    creator: Box<dyn FnMut() -> Participant>,
}

/// Terminators
impl<'a> ParticipantCreator<'a> {
    /// Sends the edited participant
    pub fn update(mut self) -> Result<Participant> {
        self.client
            .create_tournament_participant(self.tournament_id, (self.creator)())
    }
}

/// A lazy participant editor
pub struct ParticipantEditor<'a> {
    client: &'a Toornament,

    /// Tournament id in which the participant is in
    tournament_id: TournamentId,
    /// Participant's id
    id: ParticipantId,
    /// Participant editor
    editor: Box<dyn FnMut(Participant) -> Participant>,
}

/// Terminators
impl<'a> ParticipantEditor<'a> {
    /// Sends the edited participant
    pub fn update(mut self) -> Result<Participant> {
        let original = self
            .client
            .tournament_participant(self.tournament_id.clone(), self.id.clone())?;
        let edited = (self.editor)(original);
        self.client
            .update_tournament_participant(self.tournament_id, self.id, edited)
    }
}
