use ic_cdk::api::call::{CallResult, RejectionCode};

pub fn reject<T>(msg: String) -> CallResult<T> {
    Err((RejectionCode::CanisterReject, msg))
}

pub fn error<T>(msg: String) -> CallResult<T> {
    Err((RejectionCode::CanisterError, msg))
}
