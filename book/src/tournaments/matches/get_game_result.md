# Get game result

To get a result of a match game you can use `Toornament::match_game_result` method:

```rust
// Get a match game result with number "3" of a match with id = "2" of a tournament with id = "1"
let result = toornament.match_game_result(TournamentId("1".to_owned()),
                                          MatchId("2".to_owned()),
                                          GameNumber(3i64));
```

The `result` object will now contain a new `MatchResult` object.
