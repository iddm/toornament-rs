# Create participant

To create a participant of a tournament you can use `Toornament::create_tournament_participant`
method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Define a participant
    let participant = Participant::create("Test participant");
    // Create a participant for a tournament with id = "1"
    let participant = toornament.create_tournament_participant(TournamentId("1".to_owned()),
                                                               participant);
}
```

This will create a new participant and add it to the tournament with id = 1.

Creating a participant via `iter-like` interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let participant = toornament.tournaments_iter()
                                .with_id(TournamentId("1".to_owned()))
                                .participants()
                                .create(|| Participant::create("Test participant"))
                                .update();
}
```
