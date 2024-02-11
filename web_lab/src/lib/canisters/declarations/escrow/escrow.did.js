export const idlFactory = ({ IDL }) => {
  const RejectionCode = IDL.Variant({
    'NoError' : IDL.Null,
    'CanisterError' : IDL.Null,
    'SysTransient' : IDL.Null,
    'DestinationInvalid' : IDL.Null,
    'Unknown' : IDL.Null,
    'SysFatal' : IDL.Null,
    'CanisterReject' : IDL.Null,
  });
  const Result = IDL.Variant({
    'Ok' : IDL.Nat,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  return IDL.Service({
    'getPot' : IDL.Func([IDL.Nat64], [Result], []),
    'get_caller_balance' : IDL.Func([], [Result], []),
    'get_canister_balance' : IDL.Func([], [Result], []),
    'get_deposit_address' : IDL.Func([IDL.Nat64], [IDL.Vec(IDL.Nat8)], []),
    'get_value' : IDL.Func([], [IDL.Text], []),
    'place_bet' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
