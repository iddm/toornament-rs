# Get match game

To get games of a match you can use `Toornament::match_games` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    /// Get match games of a match with id = "2" of a tournament with id = "1"
    let games = toornament.match_games(TournamentId("1".to_owned()),
                                       MatchId("2".to_owned()),
                                       true);
}
```

The third boolean parameter determines should the server also return game stats in it's answer.

And via `iter-like` interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let games = toornament.tournaments_iter()
                          .with_id(TournamentId("1".to_owned()))
                          .matches()
                          .with_id(MatchId("2".to_owned()))
                          .games()
                          .collect::<Games>();
}
```
