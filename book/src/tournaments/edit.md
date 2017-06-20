# Edit a tournament

So, on the previous page we created a tournament. Let's edit some of it's parts!
Imagine, that we want to add a web-site of the tournament so the participants or just `toornament`
users may know more about the tournament. We do this simply: edit needed field of the tournament
object and then send it through the same endpoint. So, by the steps:

1. Editing a tournament object:

    ```rust,no_run
    extern crate toornament;
    use toornament::*;

    fn main() {
        let toornament = Toornament::with_application("API_TOKEN",
                                                      "CLIENT_ID",
                                                      "CLIENT_SECRET").unwrap()
                                    .timeout(5);

        // Defining a website
        let tournament_website = Some("https://toornament.com/".to_owned());

        // Creating a `Tournament` object for adding it to the service
        let mut tournament = Tournament::create(DisciplineId("wwe2k17".to_owned()),
                                                "test tournament by fx",
                                                 16,
                                                 ParticipantType::Single);

        // Some checks to be sure that our website is not set and our tournament is not public
        assert!(tournament.website.is_none());
        assert_eq!(tournament.public, false);

        // Editing fields of the object
        tournament = tournament.website(tournament_website.clone())
                               .public(true);

        // Checking everything has been done correctly
        assert_eq!(tournament.website, tournament_website);
        assert_eq!(tournament.public, true);
    }
    ```

2. Sending edited tournament object with re-assigning returned tournament from the service to our
variable so it is updated with the information from the server.

    ```rust,ignore
    // Updating our previously created tournament with new website information
    tournament = toornament.edit_tournament(tournament)?;
    ```

3. Making sure that everything has been done correctly:

    ```rust,ignore
    assert_eq!(tournament.website, tournament_website);
    assert_eq!(tournament.public, true);
    ```

So, we have just edited our tournament!

Another way to do that is via `iter`-like interface:

```rust,no_run
extern crate toornament;
use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET").unwrap()
                                .timeout(5);

    // Creating a `Tournament` object for adding it to the service
    let mut tournament = Tournament::create(DisciplineId("wwe2k17".to_owned()),
                                            "test tournament by fx",
                                             16,
                                             ParticipantType::Single);

    // Some checks to be sure that our website is not set and our tournament is not public
    assert!(tournament.website.is_none());
    assert_eq!(tournament.public, false);

    tournament = toornament.tournaments_iter()
                           .with_id(tournament.id.unwrap())
                           .edit(|mut t| t.website(Some("https://toornament.com/".to_owned()))
                                          .public(true))
                           .update().unwrap();

    // Checking everything has been done correctly
    assert_eq!(tournament.website, Some("https://toornament.com/".to_owned()));
    assert_eq!(tournament.public, true);
}
```
