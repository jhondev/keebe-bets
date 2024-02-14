mod bet;
mod bookmaker;
mod errors;
mod event;
mod ledger;
mod utils;

use std::cell::RefCell;

use bet::{BetId, BetStatus};
use bookmaker::BookMaker;
use candid::{candid_method, Nat, Principal};
use errors::reject;
use event::{EventErr, EventId};
use ic_cdk::{api::call::CallResult, caller, query, update};
use ic_ledger_types::{AccountIdentifier, Subaccount, Tokens, DEFAULT_SUBACCOUNT};
use utils::{val_auth, Convert};

#[derive(Default)]
pub struct State {
    // owner: Option<Principal>,
    maker: BookMaker,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[update(name = "placeBet")]
#[candid_method(rename = "placeBet")]
pub async fn place_bet(
    event_id: EventId,
    bet_id: BetId,
    winner: String,
    amount: u64,
) -> CallResult<()> {
    val_auth()?;
    let balance = ledger::get_bet_balance(bet_id).await?;

    if balance < amount {
        reject(format!(
            "Insufficient balance to place bet. Current balance in bet account {} ICP",
            balance
        ))?;
    }
    STATE.with(|s| {
        s.borrow_mut()
            .maker
            .place_bet(event_id, bet_id, amount, winner)
    })
}

#[update(name = "acceptBet")]
#[candid_method(rename = "acceptBet")]
pub async fn accept_bet(event_id: EventId, bet_id: BetId, winner: String) -> CallResult<()> {
    val_auth()?;
    let balance = ledger::get_bet_balance(bet_id).await?;
    STATE.with(|s| {
        let state = s.borrow();
        let bet = state.maker.get_bet(event_id, bet_id)?;
        let req = bet.amount * (bet.participants.len() + 1) as u64;
        if balance < req {
            reject(
                format!("Insufficient balance to accept bet. Current balance in bet account {} ICP and should be at least {} ICP",
                balance, req))?;
        }
        Ok(())
    })?;

    STATE.with(|s| s.borrow_mut().maker.accept_bet(event_id, bet_id, winner))
}

#[update(name = "closeBets")]
#[candid_method(rename = "closeBets")]
pub async fn close_bets(event_id: EventId) -> CallResult<()> {
    val_auth()?; // TODO: just the owner can close the bets
    STATE.with(|s| s.borrow_mut().maker.close_bets(event_id))
}

#[update(name = "settleBets")]
#[candid_method(rename = "settleBets")]
pub async fn settle_bets(event_id: EventId, winner: String) -> CallResult<()> {
    val_auth()?; // TODO: just the owner can settle the bets
    STATE.with(|s| s.borrow_mut().maker.settle_bets(event_id, &winner))
}

#[query(name = "distributePrize")]
#[candid_method(rename = "distributePrize")]
pub async fn distribute_prize(event_id: EventId, bet_id: BetId) -> CallResult<()> {
    val_auth()?; // TODO: just the owner can distribute the prize
    let winners = STATE.with(|s| s.borrow().maker.get_bet_winners(event_id, bet_id))?;
    let prize = STATE.with(|s| {
        let state = s.borrow();
        let bet = state.maker.get_bet(event_id, bet_id)?;
        if bet.status != BetStatus::Settled {
            reject(EventErr::BetNotSettled(bet.status).to_string())?;
        }
        Ok(Tokens::from_e8s(bet.prize.unwrap()))
    })?;
    for winner in winners {
        ledger::transfer_icp(
            &Subaccount::from_u64(&bet_id),
            AccountIdentifier::new(&winner, &DEFAULT_SUBACCOUNT),
            prize,
        )
        .await?;
    }
    Ok(())
}

#[update(name = "getBetBalance")]
#[candid_method(rename = "getBetBalance")]
pub async fn get_bet_balance(bet_id: BetId) -> CallResult<Nat> {
    let canister_id = ic_cdk::api::id();
    let bet_subaccount = Subaccount::from_u64(&bet_id);
    let balance = ledger::get_balance(&canister_id, &bet_subaccount).await?;
    Ok(balance.e8s().into())
}

#[update(name = "getPot")]
#[candid_method(rename = "getPot")]
pub async fn get_pot(event_id: EventId, bet_id: BetId) -> CallResult<Nat> {
    STATE.with(|s| {
        let state = s.borrow();
        let bet = state.maker.get_bet(event_id, bet_id)?;
        Ok(bet.pot.into())
    })
}

#[update(name = "getCallerBalance")]
#[candid_method(rename = "getCallerBalance")]
pub async fn get_caller_balance() -> CallResult<Nat> {
    let balance = ledger::get_balance(&caller(), &DEFAULT_SUBACCOUNT).await?;
    Ok(balance.e8s().into())
}

#[update(name = "getCanisterBalance")]
#[candid_method(rename = "getCanisterBalance")]
pub async fn get_canister_balance() -> CallResult<Nat> {
    let canister_id = ic_cdk::api::id();
    let balance = ledger::get_balance(&canister_id, &DEFAULT_SUBACCOUNT).await?;
    Ok(balance.e8s().into())
}

#[query(name = "getDepositAddress")]
#[candid_method(rename = "getDepositAddress")]
pub fn get_deposit_address(bet_id: BetId) -> AccountIdentifier {
    let canister_id = ic_cdk::api::id();
    let subaccount = Subaccount::from_u64(&bet_id);
    AccountIdentifier::new(&canister_id, &subaccount)
}

#[query(name = "getBetWinners")]
#[candid_method(rename = "getBetWinners")]
pub fn get_bet_winners(event_id: EventId, bet_id: BetId) -> CallResult<Vec<Principal>> {
    let winners = STATE.with(|s| s.borrow().maker.get_bet_winners(event_id, bet_id))?;
    Ok(winners)
}

// Create a get_candid_pointer method so that dfx can execute it to extract candid definition.
ic_cdk::export_candid!();
