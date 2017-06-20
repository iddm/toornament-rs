# Set or update match game result

To set result of a match game you can use `Toornament::update_match_game_result` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Define a result
    let result = MatchResult {
        status: MatchStatus::Completed,
        opponents: Opponents::default(),
    };
    // Set a match game result with number "3" of a match with id = "2" of a tournament with id = "1"
    let result = toornament.update_match_game_result(TournamentId("1".to_owned()),
                                                     MatchId("2".to_owned()),
                                                     GameNumber(3i64),
                                                     result,
                                                     false);
}
```

The `result` object will now contain a new `MatchResult` object.

The last, 5th parameter (boolean one) is used to specify whether it should set a result or update 
it. So, if you want to update a result rather than add it to the game, pass `true` there.

You can do this via `iter-like` interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Set a match game result with number "3" of a match with id = "2" of a tournament with id = "1"
    let game = toornament.tournaments_iter()
                         .with_id(TournamentId("1".to_owned()))
                         .matches()
                         .with_id(MatchId("2".to_owned()))
                         .games()
                         .with_number(GameNumber(3i64))
                         .result()
                         .edit(|_| MatchResult {
                             status: MatchStatus::Completed,
                             opponents: Opponents::default(),
                         })
                         .update();
}
```
