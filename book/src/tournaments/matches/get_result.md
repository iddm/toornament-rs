# Get game result

To get result of a match you can use `Toornament::match_result` method:

```rust
// Get a match result of a match with id = "2" of a tournament with id = "1"
let result = toornament.match_result(TournamentId("1".to_owned()),
                                     MatchId("2".to_owned()));
```

The `result` object will now contain a `MatchResult` object.
