# List my tournaments

You may fetch all tournaments that were created by you or associated with your account's
credentials:

**REST**:
```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get all my tournaments
    let tournaments = toornament.my_tournaments();
}
```

or via **iter-like** interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    /// Get all my tournaments
    let tournaments = toornament.tournaments_iter().my().collect::<Tournaments>();
}
```

This will return a list of tournaments associated with your account.
