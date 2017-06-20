# Get participant

To get a particular participant of a tournament you can use 
`Toornament::tournament_participant` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get a participant with id = "2" of a tournament with id = "1"
    let participant = toornament.tournament_participant(TournamentId("1".to_owned()),
                                                        ParticipantId("2".to_owned()));
}
```

`participant` will now contain a participant you requested.

Getting a participant via `iter-like` interface:

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
                                .with_id(ParticipantId("2".to_owned()))
                                .collect::<Participant>();
}
```
