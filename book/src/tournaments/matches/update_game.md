# Update match game

To update a game data of a match you can use `Toornament::update_match_game` method:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Define a game
    let mut game = Game {
        number: GameNumber(3i64),
        status: MatchStatus::Completed,
        opponents: Opponents::default(),
    };
    // Update a match game with number "3" of a match with id = "2" of a tournament with id = "1"
    let game = toornament.update_match_game(TournamentId("1".to_owned()),
                                            MatchId("2".to_owned()),
                                            GameNumber(3i64),
                                            game);
}
```

The `game` object will now contain a new `Game` object.

Via `iter-like` interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    let game = toornament.tournaments_iter()
                         .with_id(TournamentId("1".to_owned()))
                         .matches()
                         .with_id(MatchId("2".to_owned()))
                         .games()
                         .with_number(GameNumber(3i64))
                         .edit(|_| Game {
                             number: GameNumber(3i64),
                             status: MatchStatus::Completed,
                             opponents: Opponents::default(),
                         })
                         .update();
}
```
