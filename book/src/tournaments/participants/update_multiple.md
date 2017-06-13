# Update multiple participants

To update multiple participants of a tournament you can use 
`Toornament::update_tournament_participants` method:

```rust
// Define participants
let mut participants = vec![Participant::create("First participant"),
                            Participant::create("Second participant")];
// Update a participant for a tournament with id = "1"
let new_participants = toornament.update_tournament_participants(TournamentId("1".to_owned()),
                                                                 Participants(participants));
```

This will update a list participants by replacing all old participants with new ones.
