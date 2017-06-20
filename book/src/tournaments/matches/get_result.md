# Get game result

To get result of a match you can use `Toornament::match_result` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get a match result of a match with id = "2" of a tournament with id = "1"
    let result = toornament.match_result(TournamentId("1".to_owned()),
                                         MatchId("2".to_owned()));
}
```

The `result` object will now contain a `MatchResult` object.

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
                           .result()
                           .collect::<MatchResult>();
}
```
