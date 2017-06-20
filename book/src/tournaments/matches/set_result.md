# Set game result

To set result of a match you can use `Toornament::set_match_result` method:

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
    // Set match result for a match with id = "2" of a tournament with id = "1"
    let success = toornament.set_match_result(TournamentId("1".to_owned()),
                                              MatchId("2".to_owned()),
                                              result);
}
```

This will return a `Result` which can be used in usual `Rust` way:

```rust,ignore
match success {
    Ok(_) => println!("Match result has been set successfully!"),
    Err(e) => println!("Could not set match result: {}", e),
};
```

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
                           .edit(|_| MatchResult {
                               status: MatchStatus::Completed,
                               opponents: Opponents::default(),
                           })
                           .update();
}
```
