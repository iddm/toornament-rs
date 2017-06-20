# List tournament's matches

To list all the matches of a tournament you may use `Toornament::matches` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get all matches of a tournament with id = "1"
    let matches = toornament.matches(TournamentId("1".to_owned()), None, true);
}
```

You may also get a specific match of a tournament by it's id:

```rust,ignore
// Get match with match id = "2" of a tournament with id = "1"
let matches = toornament.matches(TournamentId("1".to_owned()), Some(MatchId("2".to_owned())), true);
```

The third boolean parameter specifies should the server return games field or not.

You also can get all the matches of a tournament via `iter` interface.

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get all matches of a tournament with id = "1"
    let matches = toornament.tournaments_iter()
                            .with_id(TournamentId("1".to_owned()))
                            .matches()
                            .collect::<Matches>();
}
```
