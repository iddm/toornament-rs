# Delete a tournament

Deleting a tournament is **very simple**: just call an appropriate method with tournament id:

```rust
// Deleting our tournament
println!("Deleted tournament: {:?}\n", toornament.delete_tournament(TournamentId("1".to_owned())));
```

After that you may no longer see this tournament in your organizator's webpage.
