# List tournaments

To list all the tournaments you may use `Toornament::tournaments` method:

```rust
// Getting all tournaments
let all_tournaments: Tournaments = toornament.tournaments(None, true).unwrap();
```

You also can fetch a specific tournament by it's id (*):

```rust
// Get tournament by it's id
let tournament = toornament.tournaments(Some(TournamentId("1".to_owned())), true).unwrap();
assert_eq!(tournament.0.len(), 1);
assert_eq!(tournament.0.first().unwrap().id,
Some(TournamentId("1".to_owned())));
```

The second parameter of the method is a boolean which defines should the server include streams or 
not.

**(\*)** Note: you may not get a tournament if there is no tournament with id = 1.
