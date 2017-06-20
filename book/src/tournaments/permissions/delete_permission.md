# Delete tournament's user permission

To delete a user permission attributes of a tournament you can use
`Toornament::delete_tournament_permission` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Delete a permission with id = "2" of a tournament with id = "1"
    let result = toornament.delete_tournament_permission(TournamentId("1".to_owned()),
                                                         PermissionId("2".to_owned()));
}
```

Via `iter-like`:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let result = toornament.tournaments_iter()
                           .with_id(TournamentId("1".to_owned()))
                           .permissions()
                           .with_id(PermissionId("2".to_owned()))
                           .delete();
}
```
