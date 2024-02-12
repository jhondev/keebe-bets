use candid::Principal;
use ic_cdk::{
    api::call::{CallResult, RejectionCode},
    caller,
};
use ic_ledger_types::Subaccount;

pub trait Convert {
    fn from_u64(value: &u64) -> Self;
}

impl Convert for Subaccount {
    fn from_u64(value: &u64) -> Self {
        let mut subaccount = [0u8; 32];
        subaccount[..8].copy_from_slice(&value.to_be_bytes());
        Subaccount(subaccount)
    }
}

pub trait PrincipalExt {
    fn is_anonymous(&self) -> bool;
}

impl PrincipalExt for Principal {
    fn is_anonymous(&self) -> bool {
        self == &Principal::anonymous()
    }
}

pub fn val_auth() -> CallResult<()> {
    let caller = caller();
    match caller.is_anonymous() {
        true => Err((
            RejectionCode::CanisterReject,
            "Sorry only authenticated users can access this app!".to_string(),
        )),
        false => Ok(()),
    }
}
