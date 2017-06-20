# List disciplines

To fetch all the disciplines you can use `Toornament::disciplines` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Getting all disciplines
    let all_disciplines = toornament.disciplines(None);
}
```

This will return all the available disciplines.

To get one particular discipline in details you may use the same endpoint but pass a discipline id
there:

```rust,ignore
// Get discipline by it's id
let wwe2k17_discipline = toornament.disciplines(Some(DisciplineId("wwe2k17".to_owned())));
```

Via `fancy` interface:

```rust,ignore
let all_disciplines = Disciplines::all();
let wwe2k17_discipline = Discipline::with_id(DisciplineId("wwe2k17".to_owned()));
```

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let all = toornament.disciplines_iter()
                        .all()
                        .collect::<Disciplines>();

    let wwe2k17_discipline = toornament.disciplines_iter()
                                       .with_id(DisciplineId("wwe2k17".to_owned()))
                                       .collect::<Discipline>();
}
```
