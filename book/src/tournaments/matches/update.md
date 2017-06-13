# Update match

To update a match (change it) you can use `Toornament::update_match` method:

```rust
// Define a match
let mut match_to_edit = ...;
// Edit it's number
match_to_edit.number(2u64);

match_to_edit = toornament.update_match(TournamentId("1".to_owned()),
                                        MatchId("2".to_owned()),
                                        match_to_edit)?;
```

This will edit a match with id = 2 and return the updated object.
