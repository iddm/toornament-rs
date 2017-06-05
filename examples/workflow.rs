extern crate toornament;
extern crate chrono;

use toornament::*;
use chrono::UTC;

fn workflow() -> Result<()> {
    let tournament_website = Some("https://toornament.com/".to_owned());

    let toornament = Toornament::with_application("API_TOKEN",
                                                  "CLIENT_ID",
                                                  "CLIENT_SECRET")?
                                .timeout(5);

    // Listing all the tournaments
    println!("Tournaments: {:?}\n", toornament.tournaments(None, true));
    // Listing all the disciplines
    println!("Disciplines: {:?}\n", toornament.disciplines(None));
    // Listing all the disciplines
    println!("Disciplines with id=\"wwe2k17\": {:?}\n",
             toornament.disciplines(Some(DisciplineId("wwe2k17".to_owned()))));

    // Creating a `Tournament` object for adding it to the service
    let mut tournament = Tournament::create(DisciplineId("wwe2k17".to_owned()),
                                            "test tournament by fx",
                                             16,
                                             ParticipantType::Single);
    assert!(tournament.website.is_none());
    // Sending it to the service
    tournament = toornament.edit_tournament(tournament)?;
    println!("Created tournament: {:?}\n", tournament);

    let wwe2k17_tournaments = toornament.tournaments(Some(tournament.id.clone().unwrap()),
                                                    false)?;
    let wwe2k17_t = wwe2k17_tournaments.0.first().clone().unwrap();
    assert_eq!(wwe2k17_t.id, tournament.id);

    // Setting the website and making the tournament public so we can fetch matches.
    // For making the tournament public we must also set start date
    tournament.website(tournament_website.clone())
              .date_start(Some(UTC::today().naive_utc()))
              .public(true);
    assert_eq!(tournament.website, tournament_website);
    assert_eq!(tournament.public, true);

    // Updating our previously created tournament with new website information
    tournament = toornament.edit_tournament(tournament)?;
    assert_eq!(tournament.website, tournament_website);
    assert_eq!(wwe2k17_t.id, tournament.id);

    let my_wwe_t = toornament.my_tournaments()?.0.iter().find(|t| t.id == tournament.id).unwrap().clone();
    println!("My tournaments: {:?}\n", my_wwe_t);

    // Matches are empty since we have just created our tournament
    println!("My matches: {:?}\n", toornament.matches(tournament.id.clone().unwrap(),
                                                      None,
                                                      true));
    // But let's look all the matches for wwe2k17 discipline
    println!("Matches for wwe2k17: {:?}\n",
             toornament.matches_by_discipline(DisciplineId("wwe2k17".to_owned()),
             MatchFilter::default()));

    // Let's create participants and add them to our tournament so we can create matches
    let participants = vec![Participant::create("First participant"),
                            Participant::create("Second participant")];

    // Send participants to a tournament with id = "1"
    let _ = toornament.update_tournament_participants(
        tournament.id.clone().unwrap(),
        Participants(participants))?;

    // Get matches
    println!("My matches: {:?}\n", toornament.matches(tournament.id.clone().unwrap(),
                                                      None,
                                                      true));
    

    // Deleting our tournament
    println!("Deleted tournament: {:?}\n",
             toornament.delete_tournament(tournament.id.unwrap()));

    Ok(())
}

fn main() {
    if let Err(e) = workflow() {
        println!("Error occured during the work flow: {:?}", e);
    }
}
