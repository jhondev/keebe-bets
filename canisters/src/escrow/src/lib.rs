// mod bet;
// mod bookmaker;
// mod event;
mod utils;

use std::cell::RefCell;

// use bookmaker::BookMaker;
use candid::{candid_method, CandidType, Nat, Principal};
use ic_cdk::{caller, query, update};
use ic_ledger_types::{
    AccountIdentifier,
    // BlockIndex, Memo, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};
// use serde::{Deserialize, Serialize};
use utils::principal_to_subaccount;

const ICP_FEE: u64 = 10_000;
// const ESCROW_FEE: f64 = 0.05; // 5%

#[derive(Default)]
pub struct State {
    // owner: Option<Principal>,
    // _maker: BookMaker,
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
async fn deposit_icp(caller: Principal) -> Result<Nat, DepositErr> {
    let canister_id = ic_cdk::api::id();
    // let ledger_canister_id = STATE
    //     .with(|s| s.borrow().ledger_id)
    //     .unwrap_or(MAINNET_LEDGER_CANISTER_ID);

    let account = AccountIdentifier::new(&canister_id, &principal_to_subaccount(&caller));

    let balance_args = ic_ledger_types::AccountBalanceArgs { account };
    let balance = ic_ledger_types::account_balance(MAINNET_LEDGER_CANISTER_ID, balance_args)
        .await
        .map_err(|_| DepositErr::TransferFailure)?;

    if balance.e8s() < ICP_FEE {
        return Err(DepositErr::BalanceLow);
    }

    Ok((balance.e8s() - ICP_FEE).into())
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
//         ic_ledger_types::TransferArgs {
//             memo: Memo(0),
//             amount: args.amount,
//             fee: Tokens::from_e8s(ICP_FEE),
//             from_subaccount: s.subaccount,
//             to: AccountIdentifier::new(&args.to_principal, &to_subaccount),
//             created_at_time: None,
//         }
//     });
//     ic_ledger_types::transfer(ledger_id, transfer_args)
//         .await
//         .map_err(|e| format!("failed to call ledger: {:?}", e))?
//         .map_err(|e| format!("ledger transfer error {:?}", e))
// }
