use candid::CandidType;

use crate::bet::{BetStatus, Bets};

// Id associated with the match. This id is provided by the caller.
pub type EventId = u32;

#[derive(CandidType, Clone, PartialEq, Copy)]
pub enum EventStatus {
    Upcoming,
    Live,
    Cancelled,
    Finished,
}

#[derive(CandidType)]
pub enum EventErr {
    NotFound,
    NotLive(EventStatus),
    BetNotFound,
    BetNotOpen(BetStatus),
    BetNotClosed(BetStatus),
}

#[derive(CandidType)]
pub struct Event {
    pub id: EventId,
    pub status: EventStatus,
    pub winner: Option<String>,
    pub bets: Bets,
}
