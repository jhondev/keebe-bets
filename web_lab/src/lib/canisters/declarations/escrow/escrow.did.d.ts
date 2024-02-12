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
export type Result = { 'Ok' : bigint } |
  { 'Err' : [RejectionCode, string] };
export interface _SERVICE {
  'getPot' : ActorMethod<[bigint], Result>,
  'get_caller_balance' : ActorMethod<[], Result>,
  'get_canister_balance' : ActorMethod<[], Result>,
  'get_deposit_address' : ActorMethod<[bigint], Uint8Array | number[]>,
  'get_value' : ActorMethod<[], string>,
  'place_bet' : ActorMethod<[bigint, bigint], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
