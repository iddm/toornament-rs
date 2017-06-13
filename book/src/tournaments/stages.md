# Get tournament stages

To fetch stages of a tournament you can use `Toornament::tournament_stages` method:

```rust
// Get stages of a tournament with id = "1"
let stages = toornament.tournament_stages(TournamentId("1".to_owned()));
```

This will return stages as a result.
