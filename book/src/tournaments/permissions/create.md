# Create a tournament's user permission

To create a user permission of a tournament you can use `Toornament::tournament_permissions`
method:

```rust
// Define our permission
let mut attributes = BTreeSet::new();
attributes.insert(PermissionAttribute::Register);
attributes.insert(PermissionAttribute::Edit);

let permission = Permission::create("test@mail.ru", PermissionAttributes(attributes));
// Add permission to a tournament with id = "1"
let new_permission = toornament.create_tournament_permission(TournamentId("1".to_owned()),
                                                             permission);
```

This will create a user permission and return it as a result.
