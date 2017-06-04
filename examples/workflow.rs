extern crate toornament;

use toornament::*;

fn main() {
    let toornament = Toornament::with_application("API_TOKEN", "CLIENT_ID", "CLIENT_SECRET").unwrap().timeout(5);

    // let id = TournamentId("435959567336932466".to_owned());
    // let t = toornament.tournaments(Some(id.clone()), false).unwrap();
    // println!("Tournament: {:?}", t);
    // let ps = toornament.tournament_participants(id.clone(), TournamentParticipantsFilter::default());
    // println!("Participants: {:?}", ps);
    // let mut pc = Participant::default();
    // pc.name("sd");
    // let pid = ParticipantId("442607506370207744".to_owned());
    // // let p = toornament.create_tournament_participant(id.clone(), pc);
    // // println!("Participant created: {:?}", p);
    // let p = toornament.tournament_participant(id.clone(), pid.clone());
    // println!("Participant got: {:?}", p);


    println!("Disciplines: {:?}\n", toornament.disciplines(None));
    println!("Disciplines with id=\"wwe2k17\": {:?}\n", toornament.disciplines(Some(DisciplineId("wwe2k17".to_owned()))));
    let mut tournament = Tournament::create(DisciplineId("wwe2k17".to_owned()),
                                            "test tournament by fx",
                                             16,
                                             ParticipantType::Single);
    match toornament.edit_tournament(tournament) {
        Ok(t) => { tournament = t; },
        Err(e) => panic!("Unable to create tournament: {:?}", e),
    }
    println!("Created tournament: {:?}\n", tournament);
    println!("My tournaments: {:?}\n", toornament.my_tournaments());
    println!("My matches: {:?}\n", toornament.matches(tournament.id.clone().unwrap(), None, true));
    println!("Matches for wwe2k17: {:?}\n",
             toornament.matches_by_discipline(DisciplineId("wwe2k17".to_owned()),
             MatchFilter::default()));
    println!("Deleted tournament: {:?}\n", toornament.delete_tournament(tournament.id.unwrap()));
}
