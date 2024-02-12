use std::collections::HashMap;

use candid::Principal;
use ic_cdk::api::call::{CallResult, RejectionCode};

use crate::bet::{Bet, BetId, BetStatus, ParticipantBet};
use crate::errors::reject;
use crate::event::{Event, EventErr, EventId, EventStatus};

pub type Events = HashMap<EventId, Event>;

#[derive(Default)]
pub struct BookMaker {
    pub events: Events,
}

impl BookMaker {
    pub fn place_bet(
        &mut self,
        event_id: EventId,
        bet_id: BetId,
        amount: u64,
        winner: String,
    ) -> CallResult<()> {
        let caller = ic_cdk::caller();
        let event = self.events.get_mut(&event_id);
        let bet = Bet {
            id: bet_id,
            event_id,
            creator: caller,
            amount,
            pot: amount,
            participants: vec![ParticipantBet { id: caller, winner }],
            status: BetStatus::Open,
            prize: None,
        };
        match event {
            Some(event) => {
                event.bets.insert(bet_id, bet);
            }
            None => {
                let mut event = Event {
                    id: event_id,
                    status: EventStatus::Upcoming,
                    winner: None,
                    bets: HashMap::new(),
                };
                event.bets.insert(bet_id, bet);
                self.events.insert(event_id, event);
            }
        }
        Ok(())
    }

    pub fn accept_bet(
        &mut self,
        event_id: EventId,
        bet_id: BetId,
        winner: String,
    ) -> CallResult<()> {
        let caller = ic_cdk::caller();
        let bet = self.get_bet_mut(event_id, bet_id)?;
        if bet.status != BetStatus::Open {
            reject(EventErr::BetNotOpen(bet.status).to_string())?;
        }
        bet.pot += bet.amount;
        bet.participants.push(ParticipantBet { id: caller, winner });

        Ok(())
    }

    /// Closing bets means that the event has started and is not possible
    /// to place or accept more bets related to that event
    pub fn close_bets(&mut self, event_id: EventId) -> CallResult<()> {
        let event = self.events.get_mut(&event_id);
        match event {
            Some(event) => {
                // if bets are getting closed, the event is live
                event.status = EventStatus::Live;
                for (_, bet) in event.bets.iter_mut() {
                    bet.status = BetStatus::Closed;
                }
                Ok(())
            }
            None => reject(EventErr::BetNotFound.to_string()),
        }
    }

    /// Settle bets means that the event has finished and the winner is known
    /// all event bets will be settled and the prize will be calculated and set.
    /// Distribution will be done in another step after reviewing the results
    pub fn settle_bets(&mut self, event_id: EventId, winner: &str) -> CallResult<()> {
        let event = self.events.get_mut(&event_id);
        match event {
            Some(event) => {
                if event.status != EventStatus::Live {
                    return reject(EventErr::NotLive(event.status).to_string());
                }
                event.status = EventStatus::Finished;
                event.winner = Some(winner.to_string());
                for (_, bet) in event.bets.iter_mut() {
                    bet.set_prize(winner);
                    bet.status = BetStatus::Settled;
                }
                Ok(())
            }
            None => reject(EventErr::NotFound.to_string()),
        }
    }

    pub fn get_event(&self, event_id: EventId) -> CallResult<&Event> {
        self.events.get(&event_id).ok_or((
            RejectionCode::CanisterReject,
            EventErr::NotFound.to_string(),
        ))
    }
    pub fn get_bet_mut(&mut self, event_id: EventId, bet_id: BetId) -> CallResult<&mut Bet> {
        let event = self.events.get_mut(&event_id);
        match event {
            Some(event) => {
                let bet = event.bets.get_mut(&bet_id);
                match bet {
                    Some(bet) => Ok(bet),
                    None => reject(EventErr::BetNotFound.to_string()),
                }
            }
            None => reject(EventErr::NotFound.to_string()),
        }
    }

    pub fn get_bet(&self, event_id: EventId, bet_id: BetId) -> CallResult<&Bet> {
        let event = self.get_event(event_id)?;
        event.get_bet(bet_id)
    }

    pub fn get_bet_winners(&self, event_id: EventId, bet_id: BetId) -> CallResult<Vec<Principal>> {
        let event = self.get_event(event_id)?;
        let bet = event.get_bet(bet_id)?;
        if bet.status != BetStatus::Settled {
            reject(EventErr::BetNotSettled(bet.status).to_string())?;
        }
        let winners = bet
            .participants
            .iter()
            .filter(|p| Some(p.winner.clone()) == event.winner)
            .map(|p| p.id)
            .collect();
        Ok(winners)
    }
}
