# List matches by a discipline

To get all the matches by a discipline you can use `Toornament::matches_by_discipline`:

```rust
// But let's look all the matches for wwe2k17 discipline
let matches = toornament.matches_by_discipline(DisciplineId("wwe2k17".to_owned()),
                                               MatchFilter::default());
```

`matches` will now contain a result with game matches for a `wwe2k17` game discipline. Note, that
this method accepts a filter as second parameter which can be used for filtering server's data. In
the code above this filter is filled with default values.
