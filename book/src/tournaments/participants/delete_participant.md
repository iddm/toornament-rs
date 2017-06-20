# Delete participant

To delete a participant of a tournament you can use `Toornament::delete_tournament_participant`
method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Delete a participant with id = "2" of a tournament with id = "1"
    let result = toornament.delete_tournament_participant(TournamentId("1".to_owned()),
                                                          ParticipantId("2".to_owned()));
}
```

This will delete a participant with id = 2 from the tournament.

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
                           .participants()
                           .with_id(ParticipantId("2".to_owned()))
                           .delete();
}
```
