# Delete tournament's user permission

To delete a user permission attributes of a tournament you can use
`Toornament::delete_tournament_permission` method:

```rust
// Delete a permission with id = "2" of a tournament with id = "1"
let result = toornament.delete_tournament_permission(TournamentId("1".to_owned()),
                                                     PermissionId("2".to_owned()));
```
