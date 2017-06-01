extern crate toornament;

use toornament::*;

fn main() {
    let t = Toornament::with_application("s", "s", "s");
    println!("Result: {:?}", t);

    // let t = tr.unwrap();
    // println!("Disciplines: {:?}", t.disciplines(None));
    // println!("Disciplines: {:?}", t.disciplines(Some(DisciplineId("wwe2k17".to_owned()))));
    // let tournament = Tournament::new(None,
    //                                  DisciplineId("wwe2k17".to_owned()),
    //                                  "test tournament by fx",
    //                                  TournamentStatus::Running,
    //                                  true,
    //                                  false,
    //                                  16);
    // println!("Created tournament: {:?}", t.edit_tournament(tournament));
    // println!("Deleted tournament: {:?}", t.delete_tournament(TournamentId("5846b43bfc7b7ee6188b4569".to_owned())));
    // println!("My tournaments: {:?}", t.my_tournaments());
    // println!("My matches: {:?}", t.matches(t.my_tournaments().unwrap().0.iter().next().unwrap().id.clone().unwrap(), Some(MatchId("asd".to_owned())), true));
}
