mod bet;
mod bookmaker;
mod event;
mod ledger;
mod utils;

use std::cell::RefCell;

use bookmaker::BookMaker;
use candid::{candid_method, CandidType, Nat};
use ic_cdk::{api::call::CallResult, caller, query, update};
use ic_ledger_types::{AccountIdentifier, Subaccount, Tokens, DEFAULT_SUBACCOUNT};
// use serde::{Deserialize, Serialize};
// use utils::principal_to_subaccount;
use utils::{val_auth, Convert};

#[derive(Default)]
pub struct State {
    // owner: Option<Principal>,
    _maker: BookMaker,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}

#[update]
#[candid_method(update)]
async fn place_bet(bet_id: u64, amount: u64) -> CallResult<Nat> {
    val_auth()?;
    let canister_id = ic_cdk::api::id();
    // create a subaccount for the bet in the canister Principal
    let bet_subaccount = Subaccount::from_u64(&bet_id);
    // TODO: validate balance | validate existing bet
    // let balance = ledger::get_balance(&canister_id, &principal_to_subaccount(&caller)).await?;
    ledger::transfer_icp(
        &DEFAULT_SUBACCOUNT,
        AccountIdentifier::new(&canister_id, &bet_subaccount),
        Tokens::from_e8s(amount),
    )
    .await
}

#[update(name = "getPot")]
#[candid_method(rename = "getPot")]
pub async fn get_pot(bet_id: u64) -> CallResult<Nat> {
    let canister_id = ic_cdk::api::id();
    let bet_subaccount = Subaccount::from_u64(&bet_id);
    let balance = ledger::get_balance(&canister_id, &bet_subaccount).await?;
    Ok(balance.e8s().into())
}

#[update]
#[candid_method]
pub async fn get_canister_balance() -> CallResult<Nat> {
    let canister_id = ic_cdk::api::id();
    let balance = ledger::get_balance(&canister_id, &DEFAULT_SUBACCOUNT).await?;
    Ok(balance.e8s().into())
}

#[update]
#[candid_method]
pub async fn get_caller_balance() -> CallResult<Nat> {
    let caller = caller();
    let balance = ledger::get_balance(&caller, &DEFAULT_SUBACCOUNT).await?;
    Ok(balance.e8s().into())
}

#[query]
#[candid_method]
pub fn get_value() -> String {
    // let canister_id = ic_cdk::api::id();
    let caller = caller();
    let account = AccountIdentifier::new(&caller, &DEFAULT_SUBACCOUNT);
    format!(
        "caller principal: {} \naccountidentifier: {}",
        &caller, account
    )
}

#[query]
#[candid_method]
pub fn get_deposit_address(bet_id: u64) -> AccountIdentifier {
    let canister_id = ic_cdk::api::id();
    let subaccount = Subaccount::from_u64(&bet_id);

    AccountIdentifier::new(&canister_id, &subaccount)
}

// Create a get_candid_pointer method so that dfx can execute it to extract candid definition.
ic_cdk::export_candid!();
