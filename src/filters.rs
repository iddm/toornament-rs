use participants::ParticipantId;
use tournaments::TournamentId;
use videos::VideoCategory;
use common::Date;

use std::fmt;

/// Date sorting filter
#[derive(Debug, Clone)]
pub enum DateSortFilter {
    /// Sort by date ascending
    DateAscending,
    /// Sort by date descending
    DateDescending,
}
impl fmt::Display for DateSortFilter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DateSortFilter::DateAscending => fmt.write_str("date_asc"),
            DateSortFilter::DateDescending => fmt.write_str("date_desc"),
        }
    }
}

/// Create date sorting filter
#[derive(Debug, Clone)]
pub enum CreateDateSortFilter {
    /// Sort by date ascending
    CreatedAscending,
    /// Sort by date descending
    CreatedDescending,
}
impl fmt::Display for CreateDateSortFilter {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDateSortFilter::CreatedAscending => fmt.write_str("created_asc"),
            CreateDateSortFilter::CreatedDescending => fmt.write_str("created_desc"),
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
    pub sort: Option<DateSortFilter>,
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
            sort: Some(DateSortFilter::DateAscending),
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
    builder_o!(sort, DateSortFilter);
    builder_o!(participant_id, ParticipantId);
    builder_o!(tournament_ids, Vec<TournamentId>);
    builder!(with_games, bool);
    builder_o!(before_date, Date);
    builder_o!(after_date, Date);
    builder_o!(page, i64);
}

/// A filter for tournament participants
#[derive(Debug, Clone)]
pub struct TournamentParticipantsFilter {
    /// When set to `true`, it will include the lineup of the team (works only if the participant
    /// is a team).
    pub with_lineup: bool,
    /// When set to `true`, it will include the list of custom fields for this participant.
    pub with_custom_fields: bool,
    /// Sorts the collection in a particular order. `DateAscending` sort matches from oldest to
    /// newest and `DateDescending` sort matches from newest to oldest.
    pub sort: DateSortFilter,
    /// Page requested of the list.
    pub page: i64,
}
impl Default for TournamentParticipantsFilter {
    fn default() -> TournamentParticipantsFilter {
        TournamentParticipantsFilter {
            with_lineup: false,
            sort: DateSortFilter::DateAscending,
            with_custom_fields: false,
            page: 1i64,
        }
    }
}
impl TournamentParticipantsFilter {
    builder!(with_lineup, bool);
    builder!(sort, DateSortFilter);
    builder!(with_custom_fields, bool);
    builder!(page, i64);
}

/// A filter for tournament videos
#[derive(Debug, Clone)]
pub struct TournamentVideosFilter {
    /// Category of the videos.
    pub category: Option<VideoCategory>,
    /// Sorts the collection in a particular order. `CreatedAscending` sorts the videos from older
    /// to newer; `CreatedDescending` sorts the videos from newer to older.
    pub sort: CreateDateSortFilter,
    /// Page requested of the list.
    pub page: Option<i64>,
}
impl Default for TournamentVideosFilter {
    fn default() -> TournamentVideosFilter {
        TournamentVideosFilter {
            category: None,
            sort: CreateDateSortFilter::CreatedAscending,
            page: None,
        }
    }
}
impl TournamentVideosFilter {
    builder_o!(category, VideoCategory);
    builder!(sort, CreateDateSortFilter);
    builder_o!(page, i64);
}
