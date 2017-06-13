# Update match game

To update a game data of a match you can use `Toornament::update_match_game` method:

```rust
// Define a game
let mut game = Game {
    number: GameNumber(3i64),
    status: MatchStatus::Completed,
    opponents: Opponents::default(),
};
// Update a match game with number "3" of a match with id = "2" of a tournament with id = "1"
let game = toornament.update_match_game(TournamentId("1".to_owned()),
                                        MatchId("2".to_owned()),
                                        GameNumber(3i64),
                                        game));
```

The `game` object will now contain a new `Game` object.
