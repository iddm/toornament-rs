# Create a tournament

Let's split up object creation in the rust language and tournament creation in the toornament
service. Ok, I assume, you did that.

Creation of a tournament in the service is fairly simple. All the needed information by the api is
already specified in the structure implementation so you may easily create an object of a structure
specifically for creating purposes:

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

    tournament = toornament.edit_tournament(tournament).unwrap();
    println!("Created tournament: {:?}\n", tournament);

    assert!(tournament.id.is_some());
}
```

Here we simply created a object of `Tournament` structure which we may use for sending to the
service already: however, it contains very basic information (most of it's parts is simply
unspecified), it contains enough information for creation. So, let's create a new tournament in the
`toornament`:

Here is how simple it goes. Let me explain what is going on here:

1. We created a `tournament` object through `Tournament::create` method.
2. We use it for sending it to the service through `Toornament::edit_tournament` method.
3. We then got a created tournament object from the service and re-assigned it to our variable.

So, after that we may check that our `tournament` object has an `id` field filled by the
`toornament.com` server.


So, the tournament is created now and you may see it on the organizator's web page.

You may wonder why a method for **tournament creation** is called "edit_tournament", the
reason for this is quite simple: we use same method for editing tournament and creating it. The
action depends on does the passed `Tournament` object have an `id`: if it does, the method edits
the tournament on the server, if not - creates new one.

Here is another way to create a tournament, via `iter` inteface:

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

    let result = toornament.tournaments_iter()
                           .create(|| {
                                 Tournament::create(DisciplineId("wwe2k17".to_owned()),
                                                    "test tournament by fx",
                                                    16,
                                                    ParticipantType::Single)
                           }).update();
}
```

Looks a bit simplier, right? Also, we don't call `Toornament` explicitly here, we just use a method
of `TournamentIter` structure.
