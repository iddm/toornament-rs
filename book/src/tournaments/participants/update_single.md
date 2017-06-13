# Update single participant

To update a participant of a tournament you can use `Toornament::update_tournament_participant`
method:

```rust
// At first get a participant with id = "2" of a tournament with id = "1"
let mut participant = toornament.tournament_participant(TournamentId("1".to_owned()),
                                                        ParticipantId("2".to_owned())).unwrap();
// Update the participant's name and send it
participant.name("Updated participant name here".to_owned());
let updated_participant = toornament.update_tournament_participant(
    TournamentId("1".to_owned()),
    ParticipantId("2".to_owned()),
    participant);
```

This will update a participant in the tournament and return updated participant object as a result.
