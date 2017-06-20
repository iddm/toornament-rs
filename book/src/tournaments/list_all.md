# List tournaments

To list all the tournaments you may use `Toornament::tournaments` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Getting all tournaments
    let all_tournaments: Tournaments = toornament.tournaments(None, true).unwrap();
}
```

You also can fetch a specific tournament by it's id (\*):

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get tournament by it's id
    let tournament = toornament.tournaments(Some(TournamentId("1".to_owned())), true).unwrap();
    assert_eq!(tournament.0.len(), 1);
    assert_eq!(tournament.0.first().unwrap().id, Some(TournamentId("1".to_owned())));
}
```

The second parameter of the method is a boolean which defines should the server include streams or 
not.

Another way to do that is via `iter`-like interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Getting all tournaments
    let all_tournaments = toornament.tournaments_iter()
                                    .all()
                                    .collect::<Tournaments>().unwrap();
    // Get tournament by it's id
    let tournament = toornament.tournaments_iter()
                               .with_id(TournamentId("1".to_owned()))
                               .collect::<Tournament>()
                               .unwrap();
}
```

**(\*)** Note: you may not get a tournament if there is no tournament with id = 1.
