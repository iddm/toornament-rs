use participants::ParticipantId;
use tournaments::TournamentId;
use common::Date;

use std::fmt;

#[derive(Debug, Clone)]
pub enum MatchFilterSort {
    DateAscending,
    DateDescending,
}
impl fmt::Display for MatchFilterSort {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MatchFilterSort::DateAscending => fmt.write_str("date_asc"),
            MatchFilterSort::DateDescending => fmt.write_str("date_desc"),
        }
    }
}

/// A filter for match endpoints
#[derive(Debug, Clone)]
pub struct MatchFilter {
    /// When set to `true`, returns matches from featured tournaments in the collection.
    /// When set to `false`, it returns matches from tournaments without featured. 
    /// Featured tournaments are tagged by Toornament as major tournaments for a given discipline.
    pub featured: Option<bool>,
    /// When set to `true`, returns only matches with a result.
    /// When set to `false`, returns only matches without a result.
    pub has_result: Option<bool>,
    /// Sorts the collection in a particular order. `DateAscending` sort matches from oldest to
    /// newest and `DateDescending` sort matches from newest to oldest.
    pub sort: Option<MatchFilterSort>,
    /// Returns matches that involves the given participant's id.
    pub participant_id: Option<ParticipantId>,
    /// Returns matches from the filtered tournaments.
    pub tournament_ids: Option<Vec<TournamentId>>,
    /// When set to `true`, it will include a summary of each game of the match.
    pub with_games: bool,
    /// Filter all matches scheduled before this date.
    pub before_date: Option<Date>,
    /// Filter all matches scheduled after this date.
    pub after_date: Option<Date>,
    /// Page requested of the list.
    pub page: Option<i64>,
}
impl Default for MatchFilter {
    fn default() -> MatchFilter {
        MatchFilter {
            featured: None,
            has_result: None,
            sort: Some(MatchFilterSort::DateAscending),
            participant_id: None,
            tournament_ids: None,
            with_games: false,
            before_date: None,
            after_date: None,
            page: Some(1i64),
        }
    }
}
impl MatchFilter {
    builder_o!(featured, bool);
    builder_o!(has_result, bool);
    builder_o!(sort, MatchFilterSort);
    builder_o!(participant_id, ParticipantId);
    builder_o!(tournament_ids, Vec<TournamentId>);
    builder!(with_games, bool);
    builder_o!(before_date, Date);
    builder_o!(after_date, Date);
    builder_o!(page, i64);
}
