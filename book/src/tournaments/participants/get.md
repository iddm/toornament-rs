# Get participant

To get a particular participant of a tournament you can use 
`Toornament::tournament_participant` method:

```rust
// Get a participant with id = "2" of a tournament with id = "1"
let participant = toornament.tournament_participant(TournamentId("1".to_owned()),
                                                    ParticipantId("2".to_owned()));
```

`participant` will now contain a participant you requested.
