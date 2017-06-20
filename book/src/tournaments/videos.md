# Get tournament stages

To fetch videos of a tournament you can use `Toornament::tournament_videos` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Get videos of a tournament with id = "1" with default filter
    let videos = toornament.tournament_videos(TournamentId("1".to_owned()),
                                              TournamentVideosFilter::default());
}
```

This will return videos as a result.

Fetching videos via `iter-like` interface is simple as usual:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let videos = toornament.tournaments_iter()
                           .with_id(TournamentId("1".to_owned()))
                           .videos()
                           .collect::<Videos>();
}
```
