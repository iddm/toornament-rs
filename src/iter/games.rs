use *;

/// A match games iterator
pub struct GamesIter<'a> {
    client: &'a Toornament,

    /// Fetch games of tournament with id
    tournament_id: TournamentId,
    /// Fetch games of match with id
    match_id: MatchId,
    /// Fetch games with stats
    with_stats: bool,
}

impl<'a> GamesIter<'a> {
    /// Creates new games iterator
    pub fn new(
        client: &'a Toornament,
        tournament_id: TournamentId,
        match_id: MatchId,
    ) -> GamesIter<'a> {
        GamesIter {
            client: client,
            tournament_id: tournament_id,
            match_id: match_id,
            with_stats: false,
        }
    }
}

/// Builders
impl<'a> GamesIter<'a> {
    /// Fetch games with stats
    pub fn with_stats(mut self, with_stats: bool) -> Self {
        self.with_stats = with_stats;
        self
    }
}

/// Modifiers
impl<'a> GamesIter<'a> {
    /// Fetch game with a number
    pub fn with_number(self, number: GameNumber) -> GameIter<'a> {
        GameIter {
            client: self.client,
            tournament_id: self.tournament_id,
            match_id: self.match_id,
            with_stats: self.with_stats,
            number: number,
        }
    }
}

/// Terminators
impl<'a> GamesIter<'a> {
    /// Fetch the games
    pub fn collect<T: From<Games>>(self) -> Result<T> {
        Ok(T::from(self.client.match_games(
            self.tournament_id,
            self.match_id,
            self.with_stats,
        )?))
    }
}

/// A match game iterator
pub struct GameIter<'a> {
    client: &'a Toornament,

    /// Fetch game of tournament with id
    tournament_id: TournamentId,
    /// Fetch game of match with id
    match_id: MatchId,
    /// Fetch game with stats
    with_stats: bool,
    /// Fetch game with a number
    number: GameNumber,
}

/// Modifiers
impl<'a> GameIter<'a> {
    /// Match game lazy editor
    pub fn edit<F: 'static + FnMut(Game) -> Game>(self, editor: F) -> GameEditor<'a> {
        GameEditor {
            client: self.client,
            tournament_id: self.tournament_id,
            match_id: self.match_id,
            with_stats: self.with_stats,
            number: self.number,
            editor: Box::new(editor),
        }
    }

    /// Fetch match game result
    pub fn result(self) -> GameResultIter<'a> {
        GameResultIter {
            client: self.client,
            tournament_id: self.tournament_id,
            match_id: self.match_id,
            number: self.number,
        }
    }
}

/// Terminators
impl<'a> GameIter<'a> {
    /// Fetch the game
    pub fn collect<T: From<Game>>(self) -> Result<T> {
        Ok(T::from(self.client.match_game(
            self.tournament_id,
            self.match_id,
            self.number,
            self.with_stats,
        )?))
    }
}

/// A lazy game result editor
pub struct GameEditor<'a> {
    client: &'a Toornament,

    /// Fetch match of tournament
    tournament_id: TournamentId,
    /// Fetch match with id
    match_id: MatchId,
    /// Fetch game with stats
    with_stats: bool,
    /// Fetch game with a number
    number: GameNumber,
    /// Match result editor
    editor: Box<FnMut(Game) -> Game>,
}

/// Terminators
impl<'a> GameEditor<'a> {
    /// Edits the game
    pub fn update(mut self) -> Result<Game> {
        let original = self.client.match_game(
            self.tournament_id.clone(),
            self.match_id.clone(),
            self.number,
            self.with_stats,
        )?;
        self.client.update_match_game(
            self.tournament_id,
            self.match_id,
            self.number,
            (self.editor)(original),
        )
    }
}

/// A match game result iterator
pub struct GameResultIter<'a> {
    client: &'a Toornament,

    /// Fetch match of tournament
    tournament_id: TournamentId,
    /// Fetch match with id
    match_id: MatchId,
    /// Fetch game with a number
    number: GameNumber,
}

/// Modifiers
impl<'a> GameResultIter<'a> {
    /// Game result lazy editor
    pub fn edit<F: 'static + FnMut(MatchResult) -> MatchResult>(
        self,
        editor: F,
    ) -> GameResultEditor<'a> {
        GameResultEditor {
            client: self.client,
            tournament_id: self.tournament_id,
            match_id: self.match_id,
            number: self.number,
            editor: Box::new(editor),
        }
    }
}

/// Terminators
impl<'a> GameResultIter<'a> {
    /// Fetch the game result
    pub fn collect<T: From<MatchResult>>(self) -> Result<T> {
        Ok(T::from(self.client.match_game_result(
            self.tournament_id,
            self.match_id,
            self.number,
        )?))
    }
}

/// A lazy game result editor
pub struct GameResultEditor<'a> {
    client: &'a Toornament,

    /// Fetch match of tournament
    tournament_id: TournamentId,
    /// Fetch match with id
    match_id: MatchId,
    /// Fetch game with a number
    number: GameNumber,
    /// Editor
    editor: Box<FnMut(MatchResult) -> MatchResult>,
}

/// Terminators
impl<'a> GameResultEditor<'a> {
    /// Edits the match
    pub fn update(mut self) -> Result<MatchResult> {
        let original = self.client.match_game_result(
            self.tournament_id.clone(),
            self.match_id.clone(),
            self.number,
        )?;
        self.client.update_match_game_result(
            self.tournament_id,
            self.match_id,
            self.number,
            (self.editor)(original),
            true,
        )
    }
}
