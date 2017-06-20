# Create a tournament's user permission

To create a user permission of a tournament you can use `Toornament::tournament_permissions`
method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    use std::collections::BTreeSet;

    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Define our permission
    let mut attributes = BTreeSet::new();
    attributes.insert(PermissionAttribute::Register);
    attributes.insert(PermissionAttribute::Edit);

    let permission = Permission::create("test@mail.ru", PermissionAttributes(attributes));
    // Add permission to a tournament with id = "1"
    let new_permission = toornament.create_tournament_permission(TournamentId("1".to_owned()),
                                                                 permission);
}
```

This will create a user permission and return it as a result.

Via `iter-like`:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    use std::collections::BTreeSet;

    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let permission = toornament.tournaments_iter()
                               .with_id(TournamentId("1".to_owned()))
                               .permissions()
                               .create(|| {
                                    // Define our permission
                                    let mut attributes = BTreeSet::new();
                                    attributes.insert(PermissionAttribute::Register);
                                    attributes.insert(PermissionAttribute::Edit);

                                    Permission::create("test@mail.ru",
                                                       PermissionAttributes(attributes))
                               })
                               .update();
}
```
