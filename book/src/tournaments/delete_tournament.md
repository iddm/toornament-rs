# Delete a tournament

Deleting a tournament is **very simple**: just call an appropriate method with tournament id:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Deleting our tournament
    println!("Deleted tournament: {:?}\n",
             toornament.delete_tournament(TournamentId("1".to_owned())));
}
```

After that you may no longer see this tournament in your organizator's webpage.

Another way to do that is via `iter` interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Deleting our tournament
    println!("Deleted tournament: {:?}\n", toornament.tournaments_iter()
                                                     .with_id(TournamentId("1".to_owned()))
                                                     .delete());
}
```
