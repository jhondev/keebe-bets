import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type RejectionCode = { 'NoError' : null } |
  { 'CanisterError' : null } |
  { 'SysTransient' : null } |
  { 'DestinationInvalid' : null } |
  { 'Unknown' : null } |
  { 'SysFatal' : null } |
  { 'CanisterReject' : null };
export type Result = { 'Ok' : null } |
  { 'Err' : [RejectionCode, string] };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : [RejectionCode, string] };
export type Result_2 = { 'Ok' : Array<Principal> } |
  { 'Err' : [RejectionCode, string] };
export interface _SERVICE {
  'acceptBet' : ActorMethod<[bigint, bigint, string], Result>,
  'closeBets' : ActorMethod<[bigint], Result>,
  'distributePrize' : ActorMethod<[bigint, bigint], Result>,
  'getBetBalance' : ActorMethod<[bigint], Result_1>,
  'getBetWinners' : ActorMethod<[bigint, bigint], Result_2>,
  'getCanisterBalance' : ActorMethod<[], Result_1>,
  'getDepositAddress' : ActorMethod<[bigint], Uint8Array | number[]>,
  'getPot' : ActorMethod<[bigint, bigint], Result_1>,
  'placeBet' : ActorMethod<[bigint, bigint, string, bigint], Result>,
  'settleBets' : ActorMethod<[bigint, string], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
