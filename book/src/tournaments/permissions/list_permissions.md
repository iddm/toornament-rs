# List tournament's user permissions

To list all the permissions of a tournament you can use `Toornament::tournament_permissions`
method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get permissions of a tournament with id = "1"
    let permissions = toornament.tournament_permissions(TournamentId("1".to_owned())).unwrap();
}
```

This will return a list of tournament's user permissions.

Via `iter-like`:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let permissions = toornament.tournaments_iter()
                                .with_id(TournamentId("1".to_owned()))
                                .permissions()
                                .collect::<Permissions>();
}
```
