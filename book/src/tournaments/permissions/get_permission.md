# Get tournament's user permission

To get a user permission of a tournament you can use `Toornament::tournament_permission`
method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get a permission with id = "2" of a tournament with id = "1"
    let permission = toornament.tournament_permission(TournamentId("1".to_owned()),
                                                      PermissionId("2".to_owned())).unwrap();
}
```

This will return a user permission as a result.

Via `iter-like`:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let permission = toornament.tournaments_iter()
                               .with_id(TournamentId("1".to_owned()))
                               .permissions()
                               .with_id(PermissionId("2".to_owned()))
                               .collect::<Permission>();
}
```
