# Create participant

To create a participant of a tournament you can use `Toornament::create_tournament_participant`
method:

```rust
// Define a participant
let participant = Participant::create("Test participant");
// Create a participant for a tournament with id = "1"
let participant = toornament.create_tournament_participant(TournamentId("1".to_owned()),
                                                           participant);
```

This will create a new participant and add it to the tournament with id = 1.
