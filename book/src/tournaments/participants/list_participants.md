# List tournament's participants

To fetch participants of a tournament you can use `Toornament::tournament_participants` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get participants of a tournament with id = "1" with default filter
    let participants = toornament.tournament_participants(TournamentId("1".to_owned()),
                                                          TournamentParticipantsFilter::default());
}
```

`participants` will now contain a list of participants of the tournament.

This method accepts a special filter to filter participants by some value. This example uses filter
filled with default values.


Listing participants via `iter-like` interface is simple:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let participants = toornament.tournaments_iter()
                                 .with_id(TournamentId("1".to_owned()))
                                 .participants()
                                 .collect::<Participants>();
}
```
