# List matches by a discipline

To get all the matches by a discipline you can use `Toornament::matches_by_discipline`:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // But let's look all the matches for wwe2k17 discipline
    let matches = toornament.matches_by_discipline(DisciplineId("wwe2k17".to_owned()),
                                                   MatchFilter::default());
}
```

`matches` will now contain a result with game matches for a `wwe2k17` game discipline. Note, that
this method accepts a filter as second parameter which can be used for filtering server's data. In
the code above this filter is filled with default values.

If you want to do this via `iter-like` interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let matches = toornament.disciplines_iter()
                            .with_id(DisciplineId("wwe2k17".to_owned()))
                            .matches()
                            .collect::<Matches>()
                            .unwrap();
}
```
