use std::collections::HashMap;

use crate::bet::{Bet, BetId, BetStatus, ParticipantBet};
use crate::event::{Event, EventErr, EventId, EventStatus};

pub type Events = HashMap<EventId, Event>;

#[derive(Default)]
pub struct BookMaker {
    pub events: Events,
}

impl BookMaker {
    pub fn place_bet(&mut self, event_id: EventId, bet_id: BetId, amount: u64, winner: String) {
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
    }

    pub fn accept_bet(
        &mut self,
        event_id: EventId,
        bet_id: BetId,
        winner: String,
    ) -> Result<(), EventErr> {
        let caller = ic_cdk::caller();
        let event = self.events.get_mut(&event_id);
        match event {
            Some(event) => {
                let bet = event.bets.get_mut(&bet_id);
                match bet {
                    Some(bet) => {
                        if bet.status != BetStatus::Open {
                            return Err(EventErr::BetNotOpen(bet.status));
                        }
                        bet.pot += bet.amount;
                        bet.participants.push(ParticipantBet { id: caller, winner });
                        Ok(())
                    }
                    None => Err(EventErr::NotFound),
                }
            }
            None => Err(EventErr::NotFound),
        }
    }

    pub fn close_bets(&mut self, event_id: EventId) -> Result<(), EventErr> {
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
            None => Err(EventErr::NotFound),
        }
    }

    pub fn settle_bets(&mut self, event_id: EventId, winner: &str) -> Result<(), EventErr> {
        let event = self.events.get_mut(&event_id);
        match event {
            Some(event) => {
                if event.status != EventStatus::Live {
                    return Err(EventErr::NotLive(event.status));
                }
                event.status = EventStatus::Finished;
                event.winner = Some(winner.to_owned());
                for (_, bet) in event.bets.iter_mut() {
                    let (_prize, winners) = bet.calc_prize(winner);
                    for _winner in winners {
                        // TODO: transfer the prize to the winner
                    }
                    bet.status = BetStatus::Settled;
                }
                Ok(())
            }
            None => Err(EventErr::NotFound),
        }
    }

    pub fn settle_bet(
        &mut self,
        event_id: EventId,
        bet_id: BetId,
        _winner: String,
    ) -> Result<(), EventErr> {
        let event = self.events.get_mut(&event_id);
        match event {
            Some(event) => {
                let bet = event.bets.get_mut(&bet_id);
                match bet {
                    Some(bet) => {
                        if bet.status != BetStatus::Closed {
                            return Err(EventErr::BetNotClosed(bet.status));
                        }
                        // TODO: transfer the prize to the winners
                        bet.status = BetStatus::Settled;
                        Ok(())
                    }
                    None => Err(EventErr::NotFound),
                }
            }
            None => Err(EventErr::NotFound),
        }
    }
}
