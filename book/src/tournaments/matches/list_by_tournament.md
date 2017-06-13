# List tournament's matches

To list all the matches of a tournament you may use `Toornament::matches` method:

```rust
// Get all matches of a tournament with id = "1"
let matches = toornament.matches(TournamentId("1".to_owned()), None, true);
```

You may also get a specific match of a tournament by it's id:

```rust
// Get match with match id = "2" of a tournament with id = "1"
let matches = toornament.matches(TournamentId("1".to_owned()), Some(MatchId("2".to_owned())), true);
```

The third boolean parameter specifies should the server return games field or not.
