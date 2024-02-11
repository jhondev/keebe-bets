use candid::{Nat, Principal};
use ic_cdk::{
    api::call::{CallResult, RejectionCode},
    caller,
};
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
    from_subaccount: &Subaccount,
    to: AccountIdentifier,
    amount: Tokens,
) -> CallResult<Nat> {
    // TODO: add escrow fee
    let fee = Tokens::from_e8s(ICP_FEE);
    let total = amount + fee;
    let balance = get_balance(&caller(), from_subaccount).await?;
    if balance < total {
        return Err((
            RejectionCode::CanisterError,
            format!(
                "Insufficient balance to transfer. Current balance {} ICP",
                balance
            ),
        ));
    }
    let transfer_args = ic_ledger_types::TransferArgs {
        memo: Memo(0),
        amount,
        fee,
        from_subaccount: Some(*from_subaccount),
        to,
        created_at_time: None,
    };
    let result = ic_ledger_types::transfer(MAINNET_LEDGER_CANISTER_ID, transfer_args).await;
    match result {
        Ok(r) => match r {
            Ok(a) => {
                ic_cdk::println!(
                    "Successful transfer of {} ICP from account {:?} to account {:?}",
                    amount,
                    from_subaccount,
                    to,
                );
                Ok(a.into())
            }
            Err(err) => Err((
                RejectionCode::CanisterError,
                format!("LedgerError: {}", err),
            )),
        },
        Err(err) => Err((
            RejectionCode::CanisterError,
            format!("LedgerCall: {}", err.1),
        )),
    }
}
