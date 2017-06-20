# Update single participant

To update a participant of a tournament you can use `Toornament::update_tournament_participant`
method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // At first get a participant with id = "2" of a tournament with id = "1"
    let mut participant = toornament.tournament_participant(TournamentId("1".to_owned()),
                                                            ParticipantId("2".to_owned())).unwrap();
    // Update the participant's name and send it
    participant = participant.name("Updated participant name here".to_owned());
    let updated_participant = toornament.update_tournament_participant(
        TournamentId("1".to_owned()),
        ParticipantId("2".to_owned()),
        participant);
}
```

This will update a participant in the tournament and return updated participant object as a result.

Via `iter-like` interface:

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
                                .edit(|p| p.name("Another name"))
                                .update();
}
```
