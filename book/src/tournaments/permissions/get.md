# Get tournament's user permission

To get a user permission of a tournament you can use `Toornament::tournament_permission`
method:

```rust
// Get a permission with id = "2" of a tournament with id = "1"
let permission = toornament.tournament_permission(TournamentId("1".to_owned()),
                                                  PermissionId("2".to_owned())).unwrap();
```

This will return a user permission as a result.
