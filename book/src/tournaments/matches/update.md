# Update match

To update a match (change it) you can use `Toornament::update_match` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Define a match
    let mut match_to_edit = toornament.matches(TournamentId("1".to_owned()),
                                               Some(MatchId("2".to_owned())),
                                               true).unwrap().0.first().unwrap().to_owned();
    // Edit it's number
    match_to_edit = match_to_edit.number(2u64);

    match_to_edit = toornament.update_match(TournamentId("1".to_owned()),
                                            MatchId("2".to_owned()),
                                            match_to_edit).unwrap();
}
```

This will edit a match with id = 2 and return the updated object.

This can also be done via `fancy` interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Define a match
    let mut match_to_edit = toornament.tournaments_iter()
                                      .with_id(TournamentId("1".to_owned()))
                                      .matches()
                                      .with_id(MatchId("2".to_owned()))
                                      .edit(|m| m.number(3u64))
                                      .update();
}
```
