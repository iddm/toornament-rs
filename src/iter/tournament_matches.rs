use crate::*;
use iter::games::GamesIter;

/// A tournament matches iterator
pub struct TournamentMatchesIter<'a> {
    client: &'a Toornament,

    /// Fetch matches of tournament
    tournament_id: TournamentId,
    /// Fetch games with the match
    with_games: bool,
}
impl<'a> TournamentMatchesIter<'a> {
    /// Creates new match iterator
    pub fn new(client: &'a Toornament, tournament_id: TournamentId) -> TournamentMatchesIter {
        TournamentMatchesIter {
            client,
            tournament_id,
            with_games: false,
        }
    }
}

/// Builders
impl<'a> TournamentMatchesIter<'a> {
    /// Fetch match games
    pub fn with_games(mut self, with_games: bool) -> Self {
        self.with_games = with_games;
        self
    }

    /// Fetch match by tournament id
    pub fn of_tournament(mut self, id: TournamentId) -> Self {
        self.tournament_id = id;
        self
    }
}

/// Modifiers
impl<'a> TournamentMatchesIter<'a> {
    /// Get a match with id
    pub fn with_id(self, match_id: MatchId) -> TournamentMatchIter<'a> {
        TournamentMatchIter {
            client: self.client,
            tournament_id: self.tournament_id,
            match_id,
            with_games: self.with_games,
        }
    }
}

/// Terminators
impl<'a> TournamentMatchesIter<'a> {
    /// Fetch matches
    pub fn collect<T: From<Matches>>(self) -> Result<T> {
        Ok(T::from(self.client.matches(
            self.tournament_id,
            None,
            self.with_games,
        )?))
    }
}

/// A tournament match iterator
pub struct TournamentMatchIter<'a> {
    client: &'a Toornament,

    /// Fetch match of tournament
    tournament_id: TournamentId,
    /// Fetch match with id
    match_id: MatchId,
    /// Fetch games with the match
    with_games: bool,
}
impl<'a> TournamentMatchIter<'a> {
    /// Creates new tournament match iter
    pub fn new(
        client: &'a Toornament,
        tournament_id: TournamentId,
        match_id: MatchId,
        with_games: bool,
    ) -> TournamentMatchIter<'a> {
        TournamentMatchIter {
            client,
            tournament_id,
            match_id,
            with_games,
        }
    }
}

/// Modifiers
impl<'a> TournamentMatchIter<'a> {
    /// Tournament match lazy editor
    pub fn edit<F: 'static + FnMut(Match) -> Match>(self, editor: F) -> TournamentMatchEditor<'a> {
        TournamentMatchEditor {
            client: self.client,
            tournament_id: self.tournament_id,
            match_id: self.match_id,
            with_games: self.with_games,
            editor: Box::new(editor),
        }
    }

    /// Fetch match result
    pub fn result(self) -> TournamentMatchResultIter<'a> {
        TournamentMatchResultIter {
            client: self.client,
            tournament_id: self.tournament_id,
            match_id: self.match_id,
        }
    }

    /// Return games of this match
    pub fn games(self) -> GamesIter<'a> {
        GamesIter::new(self.client, self.tournament_id, self.match_id)
    }
}

/// Terminators
impl<'a> TournamentMatchIter<'a> {
    /// Fetch the match
    pub fn collect<T: From<Match>>(self) -> Result<T> {
        let matches = self.client.matches(
            self.tournament_id.clone(),
            Some(self.match_id.clone()),
            self.with_games,
        )?;
        match matches.0.first() {
            Some(m) => Ok(T::from(m.to_owned())),
            None => Err(Error::Iter(IterError::NoSuchMatch(
                self.tournament_id,
                self.match_id,
            ))),
        }
    }
}

/// A tournament match result iterator
pub struct TournamentMatchResultIter<'a> {
    client: &'a Toornament,

    /// Fetch match of tournament
    tournament_id: TournamentId,
    /// Fetch match with id
    match_id: MatchId,
}

/// Modifiers
impl<'a> TournamentMatchResultIter<'a> {
    /// Tournament match result lazy editor
    pub fn edit<F: 'static + FnMut(MatchResult) -> MatchResult>(
        self,
        editor: F,
    ) -> TournamentMatchResultEditor<'a> {
        TournamentMatchResultEditor {
            client: self.client,
            tournament_id: self.tournament_id,
            match_id: self.match_id,
            editor: Box::new(editor),
        }
    }
}

/// Terminators
impl<'a> TournamentMatchResultIter<'a> {
    /// Fetch the match result
    pub fn collect<T: From<MatchResult>>(self) -> Result<T> {
        Ok(T::from(
            self.client
                .match_result(self.tournament_id, self.match_id)?,
        ))
    }
}

/// A lazy match result editor
pub struct TournamentMatchResultEditor<'a> {
    client: &'a Toornament,

    /// Fetch match of tournament
    tournament_id: TournamentId,
    /// Fetch match with id
    match_id: MatchId,
    /// Match result editor
    editor: Box<dyn FnMut(MatchResult) -> MatchResult>,
}

/// Terminators
impl<'a> TournamentMatchResultEditor<'a> {
    /// Adds or edits the match result
    pub fn update(mut self) -> Result<MatchResult> {
        let original = self
            .client
            .match_result(self.tournament_id.clone(), self.match_id.clone())?;
        self.client
            .set_match_result(self.tournament_id, self.match_id, (self.editor)(original))
    }
}

/// A lazy tournament match editor
pub struct TournamentMatchEditor<'a> {
    client: &'a Toornament,

    /// Fetch match of tournament
    tournament_id: TournamentId,
    /// Fetch match with id
    match_id: MatchId,
    /// Fetch games with the match
    with_games: bool,
    /// Editor
    editor: Box<dyn FnMut(Match) -> Match>,
}

/// Terminators
impl<'a> TournamentMatchEditor<'a> {
    /// Edits the match
    pub fn update(mut self) -> Result<Match> {
        let matches = self.client.matches(
            self.tournament_id.clone(),
            Some(self.match_id.clone()),
            self.with_games,
        )?;
        let original = match matches.0.first() {
            Some(m) => m.to_owned(),
            None => {
                return Err(Error::Iter(IterError::NoSuchMatch(
                    self.tournament_id,
                    self.match_id,
                )))
            }
        };
        self.client
            .update_match(self.tournament_id, self.match_id, (self.editor)(original))
    }
}
