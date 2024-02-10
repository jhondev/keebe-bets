use std::collections::HashMap;

use candid::{CandidType, Principal};

use crate::event::EventId;

pub type BetId = u32;
pub type ParticipantId = Principal;
pub type Bets = HashMap<BetId, Bet>;

#[derive(CandidType, PartialEq, Copy, Clone)]
pub enum BetStatus {
    Open,
    Cancelled,
    Closed,
    Settled,
}

#[derive(CandidType, Clone)]
pub struct ParticipantBet {
    pub id: ParticipantId,
    pub winner: String,
}

#[derive(CandidType)]
pub struct Bet {
    pub id: BetId,
    pub event_id: EventId,
    pub creator: Principal,
    pub amount: u64,
    pub pot: u64,
    pub participants: Vec<ParticipantBet>,
    pub status: BetStatus,
}

impl Bet {
    pub fn calc_prize(&self, winner: &str) -> (u64, Vec<&ParticipantId>) {
        let mut prize = 0;
        let mut winners = Vec::<&ParticipantId>::new();
        for participant in &self.participants {
            if participant.winner == winner {
                prize += self.amount;
                winners.push(&participant.id);
            }
        }
        prize /= winners.len() as u64;
        (prize, winners)
    }
}
