use std::fmt;

use candid::CandidType;
use ic_cdk::api::call::{CallResult, RejectionCode};

use crate::bet::{Bet, BetId, BetStatus, Bets};

// Id associated with the match. This id is provided by the caller.
pub type EventId = u64;

#[derive(CandidType, Clone, PartialEq, Copy, Debug)]
pub enum EventStatus {
    Upcoming,
    Live,
    // Cancelled,
    Finished,
}

#[derive(CandidType, Debug)]
pub enum EventErr {
    NotFound,
    NotLive(EventStatus),
    BetNotFound,
    BetNotOpen(BetStatus),
    BetNotSettled(BetStatus),
}

impl fmt::Display for EventErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(CandidType)]
pub struct Event {
    pub id: EventId,
    pub status: EventStatus,
    pub winner: Option<String>,
    pub bets: Bets,
}

impl Event {
    pub fn get_bet(&self, bet_id: BetId) -> CallResult<&Bet> {
        self.bets.get(&bet_id).ok_or((
            RejectionCode::CanisterReject,
            EventErr::BetNotFound.to_string(),
        ))
    }
}
