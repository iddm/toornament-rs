# List tournament's user permissions

To list all the permissions of a tournament you can use `Toornament::tournament_permissions`
method:

```rust
// Get permissions of a tournament with id = "1"
let permissions = toornament.tournament_permissions(TournamentId("1".to_owned())).unwrap();
```

This will return a list of tournament's user permissions.
