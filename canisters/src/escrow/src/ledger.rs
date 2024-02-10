use candid::{Nat, Principal};
use ic_cdk::api::call::{CallResult, RejectionCode};
use ic_ledger_types::{
    account_balance, AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Tokens,
    MAINNET_LEDGER_CANISTER_ID,
};

const ICP_FEE: u64 = 10_000;
// const ESCROW_FEE: f64 = 0.05; // 5%

pub async fn get_balance(owner: &Principal, subaccount: &Subaccount) -> CallResult<Tokens> {
    let account = AccountIdentifier::new(owner, subaccount);
    let balance_args = AccountBalanceArgs { account };
    account_balance(MAINNET_LEDGER_CANISTER_ID, balance_args).await
}

pub async fn transfer_icp(
    from_subaccount: Option<Subaccount>,
    to: AccountIdentifier,
    amount: Tokens,
) -> CallResult<Nat> {
    // TODO: add escrow fee
    let fee = Tokens::from_e8s(ICP_FEE);
    if amount < fee {
        return Err((
            RejectionCode::CanisterError,
            format!("BalanceLow: {}", amount),
        ));
    }
    let amount = amount - fee;
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount,
        fee,
        from_subaccount,
        to,
        created_at_time: None,
    };
    let _ = ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, transfer_args).await?;

    ic_cdk::println!(
        "Transfer of {} ICP from account {:?} to account {:?}",
        amount,
        from_subaccount,
        to,
    );

    Ok(amount.e8s().into())
}
