# List tournament's participants

To fetch participants of a tournament you can use `Toornament::tournament_participants` method:

```rust
// Get participants of a tournament with id = "1" with default filter
let participants = toornament.tournament_participants(TournamentId("1".to_owned()),
                                                      TournamentParticipantsFilter::default());
```

`participants` will now contain a list of participants of the tournament.

This method accepts a special filter to filter participants by some value. This example uses filter
filled with default values.
