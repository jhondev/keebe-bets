mod bet;
mod bookmaker;
mod event;
mod ledger;
mod utils;

use std::cell::RefCell;

use bookmaker::BookMaker;
use candid::{candid_method, CandidType, Nat};
use ic_cdk::{api::call::CallResult, caller, query, update};
use ic_ledger_types::{AccountIdentifier, DEFAULT_SUBACCOUNT};
// use serde::{Deserialize, Serialize};
use utils::principal_to_subaccount;

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
async fn deposit_icp() -> CallResult<Nat> {
    let canister_id = ic_cdk::api::id();
    let caller = caller();

    let balance = ledger::get_balance(&canister_id, &principal_to_subaccount(&caller)).await?;
    ledger::transfer_icp(
        Some(principal_to_subaccount(&caller)),
        AccountIdentifier::new(&canister_id, &DEFAULT_SUBACCOUNT),
        balance,
    )
    .await
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
    let canister_id = ic_cdk::api::id();
    let balance = ledger::get_balance(&canister_id, &principal_to_subaccount(&caller())).await?;
    Ok(balance.e8s().into())
}

#[query]
#[candid_method]
pub fn get_value() -> String {
    // let canister_id = ic_cdk::api::id();
    let canister_id = caller();
    canister_id.to_text() + " | 1"
}

#[query]
#[candid_method]
pub fn get_deposit_address() -> AccountIdentifier {
    let canister_id = ic_cdk::api::id();
    let subaccount = principal_to_subaccount(&caller());

    AccountIdentifier::new(&canister_id, &subaccount)
}

// #[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
// pub struct TransferArgs {
//     amount: Tokens,
//     to_principal: Principal,
//     to_subaccount: Option<Subaccount>,
// }
// #[update]
// #[candid_method(update)]
// async fn transfer(args: TransferArgs) -> Result<BlockIndex, String> {
//     ic_cdk::println!(
//         "Transferring {} tokens to principal {} subaccount {:?}",
//         &args.amount,
//         &args.to_principal,
//         &args.to_subaccount
//     );
//     let ledger_id = STATE
//         .with(|s| s.borrow().ledger_id)
//         .unwrap_or(MAINNET_LEDGER_CANISTER_ID);
//     let to_subaccount = args.to_subaccount.unwrap_or(DEFAULT_SUBACCOUNT);
//     let transfer_args = STATE.with(|s| {
//         let s = s.borrow();
//         TransferArgs {
//             memo: Memo(0),
//             amount: args.amount,
//             fee: Tokens::from_e8s(ICP_FEE),
//             from_subaccount: s.subaccount,
//             to: AccountIdentifier::new(&args.to_principal, &to_subaccount),
//             created_at_time: None,
//         }
//     });
//     transfer(ledger_id, transfer_args)
//         .await
//         .map_err(|e| format!("failed to call ledger: {:?}", e))?
//         .map_err(|e| format!("ledger transfer error {:?}", e))
// }

// Create a get_candid_pointer method so that dfx can execute it to extract candid definition.
ic_cdk::export_candid!();
