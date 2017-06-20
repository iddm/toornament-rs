# Get game result

To get a result of a match game you can use `Toornament::match_game_result` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get a match game result with number "3" of a match with id = "2" of a tournament with id = "1"
    let result = toornament.match_game_result(TournamentId("1".to_owned()),
                                              MatchId("2".to_owned()),
                                              GameNumber(3i64));
}
```

The `result` object will now contain a new `MatchResult` object.

Via `iter-like` interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let result = toornament.tournaments_iter()
                           .with_id(TournamentId("1".to_owned()))
                           .matches()
                           .with_id(MatchId("2".to_owned()))
                           .games()
                           .with_number(GameNumber(3i64))
                           .result()
                           .collect::<MatchResult>();
}
```
