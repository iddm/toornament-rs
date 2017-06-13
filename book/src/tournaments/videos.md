# Get tournament stages

To fetch videos of a tournament you can use `Toornament::tournament_videos` method:

```rust
// Get videos of a tournament with id = "1" with default filter
let videos = toornament.tournament_videos(TournamentId("1".to_owned()),
                                          TournamentVideosFilter::default());
```

This will return videos as a result.
