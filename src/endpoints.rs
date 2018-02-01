use ::*;

const API_BASE: &'static str = "https://api.toornament.com";

#[derive(Debug, Clone)]
pub enum Endpoint {
    OauthToken,
    AllDisciplines,
    DisciplineById(DisciplineId),
    AllTournaments {
        with_streams: bool,
    },
    MyTournaments,
    TournamentByIdGet {
        tournament_id: TournamentId,
        with_streams: bool,
    },
    TournamentByIdUpdate(TournamentId),
    TournamentCreate,
    MatchesByTournament {
        tournament_id: TournamentId,
        with_games: bool,
    },
    MatchesByDiscipline {
        discipline_id: DisciplineId,
        filter: MatchFilter,
    },
    MatchByIdGet {
        tournament_id: TournamentId,
        match_id: MatchId,
        with_games: bool,
    },
    MatchByIdUpdate {
        tournament_id: TournamentId,
        match_id: MatchId,
    },
    MatchResult(TournamentId, MatchId),
    MatchGames {
        tournament_id: TournamentId,
        match_id: MatchId,
        with_stats: bool,
    },
    MatchGameByNumberGet {
        tournament_id: TournamentId,
        match_id: MatchId,
        game_number: GameNumber,
        with_stats: bool,
    },
    MatchGameByNumberUpdate {
        tournament_id: TournamentId,
        match_id: MatchId,
        game_number: GameNumber,
    },
    MatchGameResultGet {
        tournament_id: TournamentId,
        match_id: MatchId,
        game_number: GameNumber,
    },
    MatchGameResultUpdate {
        tournament_id: TournamentId,
        match_id: MatchId,
        game_number: GameNumber,
        update_match: bool,
    },
    Participants {
        tournament_id: TournamentId,
        filter: TournamentParticipantsFilter,
    },
    ParticipantCreate(TournamentId),
    ParticipantsUpdate(TournamentId),
    ParticipantById(TournamentId, ParticipantId),
    Permissions(TournamentId),
    PermissionById(TournamentId, PermissionId),
    Stages(TournamentId),
    Videos {
        tournament_id: TournamentId,
        filter: TournamentVideosFilter,
    },
}

impl ::std::fmt::Display for Endpoint {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let address;
        match *self {
            Endpoint::OauthToken => address = format!("/oauth/v2/token"),
            Endpoint::AllDisciplines => address = format!("/v1/disciplines"),
            Endpoint::DisciplineById(ref id) => address = format!("/v1/disciplines/{}", id.0),
            Endpoint::AllTournaments { with_streams } => {
                address = format!(
                    "/v1/tournaments?with_streams={}",
                    if with_streams { "1" } else { "0" }
                )
            }
            Endpoint::MyTournaments => address = format!("/v1/me/tournaments"),
            Endpoint::TournamentByIdGet {
                ref tournament_id,
                with_streams,
            } => {
                address = format!(
                    "/v1/tournaments/{}?with_streams={}",
                    tournament_id.0,
                    if with_streams { "1" } else { "0" }
                )
            }
            Endpoint::TournamentByIdUpdate(ref tournament_id) => {
                address = format!("/v1/tournaments/{}", tournament_id.0)
            }
            Endpoint::TournamentCreate => address = format!("/v1/tournaments"),
            Endpoint::MatchesByTournament {
                ref tournament_id,
                with_games,
            } => {
                address = format!(
                    "/v1/tournaments/{}/matches?with_games={}",
                    tournament_id.0,
                    if with_games { "1" } else { "0" }
                )
            }
            Endpoint::MatchByIdGet {
                ref tournament_id,
                ref match_id,
                with_games,
            } => {
                address = format!(
                    "/v1/tournaments/{}/matches/{}?with_games={}",
                    tournament_id.0,
                    match_id.0,
                    if with_games { "1" } else { "0" }
                )
            }
            Endpoint::MatchByIdUpdate {
                ref tournament_id,
                ref match_id,
            } => address = format!("/v1/tournaments/{}/matches/{}", tournament_id.0, match_id.0),
            Endpoint::MatchesByDiscipline {
                ref discipline_id,
                ref filter,
            } => {
                address = format!(
                    "/v1/disciplines/{}/matches?{}",
                    discipline_id.0,
                    match_filter(filter.clone())
                )
            }
            Endpoint::MatchResult(ref tournament_id, ref match_id) => {
                address = format!(
                    "/v1/tournaments/{}/matches/{}/result",
                    tournament_id.0, match_id.0
                )
            }
            Endpoint::MatchGames {
                ref tournament_id,
                ref match_id,
                with_stats,
            } => {
                address = format!(
                    "/v1/tournaments/{}/matches/{}/games?with_stats={}",
                    tournament_id.0,
                    match_id.0,
                    if with_stats { "1" } else { "0" }
                )
            }
            Endpoint::MatchGameByNumberGet {
                ref tournament_id,
                ref match_id,
                ref game_number,
                with_stats,
            } => {
                address = format!(
                    "/v1/tournaments/{}/matches/{}/games/{}?with_stats={}",
                    tournament_id.0,
                    match_id.0,
                    game_number.0,
                    if with_stats { "1" } else { "0" }
                )
            }
            Endpoint::MatchGameByNumberUpdate {
                ref tournament_id,
                ref match_id,
                ref game_number,
            } => {
                address = format!(
                    "/v1/tournaments/{}/matches/{}/games/{}",
                    tournament_id.0, match_id.0, game_number.0
                )
            }
            Endpoint::MatchGameResultGet {
                ref tournament_id,
                ref match_id,
                ref game_number,
            } => {
                address = format!(
                    "/v1/tournaments/{}/matches/{}/games/{}/result",
                    tournament_id.0, match_id.0, game_number.0
                )
            }
            Endpoint::MatchGameResultUpdate {
                ref tournament_id,
                ref match_id,
                ref game_number,
                update_match,
            } => {
                address = format!(
                    "/v1/tournaments/{}/matches/{}/games/{}/result?update_match={}",
                    tournament_id.0,
                    match_id.0,
                    game_number.0,
                    if update_match { "1" } else { "0" }
                )
            }
            Endpoint::Participants {
                ref tournament_id,
                ref filter,
            } => {
                address = format!(
                    "/v1/tournaments/{}/participants?{}",
                    tournament_id.0,
                    tournament_participants(filter.clone())
                )
            }
            Endpoint::ParticipantCreate(ref tournament_id) => {
                address = format!("/v1/tournaments/{}/participants", tournament_id.0)
            }
            Endpoint::ParticipantsUpdate(ref tournament_id) => {
                address = format!("/v1/tournaments/{}/participants", tournament_id.0)
            }
            Endpoint::ParticipantById(ref tournament_id, ref participant_id) => {
                address = format!(
                    "/v1/tournaments/{}/participants/{}",
                    tournament_id.0, participant_id.0
                )
            }
            Endpoint::Permissions(ref tournament_id) => {
                address = format!("/v1/tournaments/{}/permissions", tournament_id.0)
            }
            Endpoint::PermissionById(ref tournament_id, ref permission_id) => {
                address = format!(
                    "/v1/tournaments/{}/permissions/{}",
                    tournament_id.0, permission_id.0
                )
            }
            Endpoint::Stages(ref tournament_id) => {
                address = format!("/v1/tournaments/{}/stages", tournament_id.0)
            }
            Endpoint::Videos {
                ref tournament_id,
                ref filter,
            } => {
                address = format!(
                    "/v1/tournaments/{}/videos?{}",
                    tournament_id.0,
                    tournament_videos(filter.clone())
                )
            }
        };

        fmt.write_str(&format!("{}{}", API_BASE, address))
    }
}

fn match_filter(f: MatchFilter) -> String {
    let mut out = Vec::new();
    match f.featured {
        Some(f) => out.push(format!("featured={}", if f { 1 } else { 0 })),
        None => {}
    }
    match f.has_result {
        Some(r) => out.push(format!("has_result={}", if r { 1 } else { 0 })),
        None => {}
    }
    match f.sort {
        Some(s) => out.push(format!("sort={}", s.to_string())),
        None => {}
    }
    match f.participant_id {
        Some(i) => out.push(format!("participant_id={}", i.0)),
        None => {}
    }
    match f.tournament_ids {
        Some(ref i) => out.push(format!(
            "tournament_ids={}",
            i.iter()
                .map(|i| i.0.as_str())
                .collect::<Vec<&str>>()
                .join(",")
        )),
        None => {}
    }
    out.push(format!("with_games={}", if f.with_games { 1 } else { 0 }));
    match f.before_date {
        Some(d) => out.push(format!("before_date={}", d)),
        None => {}
    }
    match f.after_date {
        Some(d) => out.push(format!("after_date={}", d)),
        None => {}
    }
    match f.page {
        Some(p) => out.push(format!("page={}", p)),
        None => {}
    }
    out.join("&")
}

fn tournament_participants(f: TournamentParticipantsFilter) -> String {
    format!(
        "with_lineup={}&with_custom_fields={}&sort={}&page={}",
        f.with_lineup as u64,
        f.with_custom_fields as u64,
        f.sort.to_string(),
        f.page
    )
}

fn tournament_videos(f: TournamentVideosFilter) -> String {
    let mut out = Vec::new();
    match f.category {
        Some(c) => out.push(format!("category={}", c.to_string())),
        None => {}
    }
    out.push(format!("sort={}", f.sort.to_string()));
    match f.page {
        Some(p) => out.push(format!("page={}", p)),
        None => {}
    }
    out.join("&")
}

#[cfg(test)]
mod tests {
    use MatchFilter;
    use endpoints::match_filter;

    #[test]
    fn test_match_filter_to_get_string() {
        let f = MatchFilter::default()
            .featured(true)
            .has_result(true)
            .page(2i64);
        assert_eq!(
            match_filter(f),
            "featured=1&has_result=1&sort=date_asc&with_games=0&page=2"
        );
    }

}
