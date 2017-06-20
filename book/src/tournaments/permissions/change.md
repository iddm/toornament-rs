# Edit tournament's user permission attributes

To edit a user permission attributes of a tournament you can use
`Toornament::update_tournament_permission_attributes` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    use std::collections::BTreeSet;

    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Define our permission attributes
    let mut attributes = BTreeSet::new();
    attributes.insert(PermissionAttribute::Register);
    attributes.insert(PermissionAttribute::Edit);

    // Update attributes of a permission with id = "2" of a tournament with id = "1"
    let permission = toornament.update_tournament_permission_attributes(
        TournamentId("1".to_owned()),
        PermissionId("2".to_owned()),
        PermissionAttributes(attributes));
}
```

This will return a new user permission as a result.

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
                               .with_id(PermissionId("2".to_owned()))
                               .attributes()
                               .edit(|_| {
                                   // Define our permission attributes
                                   let mut attributes = BTreeSet::new();
                                   attributes.insert(PermissionAttribute::Register);
                                   attributes.insert(PermissionAttribute::Edit);
                                   PermissionAttributes(attributes)
                               })
                               .update();
}
```
