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
    'Ok' : IDL.Null,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result_1 = IDL.Variant({
    'Ok' : IDL.Nat,
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Principal),
    'Err' : IDL.Tuple(RejectionCode, IDL.Text),
  });
  return IDL.Service({
    'acceptBet' : IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Text], [Result], []),
    'closeBets' : IDL.Func([IDL.Nat64], [Result], []),
    'distributePrize' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result], []),
    'getBetBalance' : IDL.Func([IDL.Nat64], [Result_1], []),
    'getBetWinners' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result_2], []),
    'getCallerAddress' : IDL.Func([], [IDL.Text], []),
    'getCallerBalance' : IDL.Func([], [Result_1], []),
    'getCanisterBalance' : IDL.Func([], [Result_1], []),
    'getDepositAddress' : IDL.Func([IDL.Nat64], [IDL.Vec(IDL.Nat8)], []),
    'getPot' : IDL.Func([IDL.Nat64, IDL.Nat64], [Result_1], []),
    'placeBet' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Text, IDL.Nat64],
        [Result],
        [],
      ),
    'settleBets' : IDL.Func([IDL.Nat64, IDL.Text], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
