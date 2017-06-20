# Get tournament stages

To fetch stages of a tournament you can use `Toornament::tournament_stages` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get stages of a tournament with id = "1"
    let stages = toornament.tournament_stages(TournamentId("1".to_owned()));
}
```

This will return stages as a result.

Fetching stages via `iter-like` interface is simple as usual:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let stages = toornament.tournaments_iter()
                           .with_id(TournamentId("1".to_owned()))
                           .stages()
                           .collect::<Stages>();
}
```
