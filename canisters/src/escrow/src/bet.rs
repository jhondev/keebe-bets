use std::collections::HashMap;

use candid::{CandidType, Principal};

use crate::event::EventId;

pub type BetId = u64;
pub type ParticipantId = Principal;
pub type Bets = HashMap<BetId, Bet>;

#[derive(CandidType, PartialEq, Copy, Clone, Debug)]
pub enum BetStatus {
    Open,
    // Cancelled,
    Closed,
    Settled,
}

#[derive(CandidType, Clone)]
pub struct ParticipantBet {
    pub id: ParticipantId,
    pub winner: String,
}

#[derive(CandidType, Clone)]
pub struct Bet {
    pub id: BetId,
    pub event_id: EventId,
    pub creator: Principal,
    pub amount: u64,
    pub pot: u64,
    pub participants: Vec<ParticipantBet>,
    pub status: BetStatus,
    pub prize: Option<u64>,
}

impl Bet {
    pub fn set_prize(&mut self, winner: &str) {
        let mut prize = 0;
        let mut winners = 0;
        for participant in &self.participants {
            if participant.winner == winner {
                prize += self.amount;
                winners += 1;
            }
        }
        prize /= winners;
        self.prize = Some(prize);
    }
}
