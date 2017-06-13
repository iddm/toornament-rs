# Get match game

To get games of a match you can use `Toornament::match_games` method:

```rust
/// Get match games of a match with id = "2" of a tournament with id = "1"
let games = toornament.match_games(TournamentId("1".to_owned()),
                                   MatchId("2".to_owned()),
                                   true);
```

The third boolean parameter determines should the server also return game stats in it's answer.
