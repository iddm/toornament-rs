# List disciplines

To fetch all the disciplines you can use `Toornament::disciplines` method:

```rust
// Getting all disciplines
let all_disciplines = toornament.disciplines(None);
```

This will return all the available disciplines.

To get one particular discipline in details you may use the same endpoint but pass a discipline id
there:

```rust
// Get discipline by it's id
let wwe2k17_discipline = toornament.disciplines(Some(DisciplineId("wwe2k17".to_owned())));
```
