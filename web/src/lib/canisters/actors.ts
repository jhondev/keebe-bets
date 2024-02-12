import {
	type ActorSubclass,
	type ActorConfig,
	type HttpAgentOptions,
	type Agent
} from '@dfinity/agent';
import { createActor } from './actor.js';
import { PUBLIC_CANISTER_ID_ESCROW, PUBLIC_CANISTER_ID_LEDGER } from '$env/static/public';
import {
	idlFactory as escrowIdlFactory,
	type _SERVICE as _ESCROW_SERVICE
} from './declarations/escrow/escrow.did.js';
import {
	idlFactory as ledgerIdlFactory,
	type _SERVICE as _LEDGER_SERVICE
} from './declarations/ledger/ledger.did.js';

declare interface CreateActorOptions {
	agent?: Agent;
	agentOptions?: HttpAgentOptions;
	actorOptions?: ActorConfig;
}

export const createEscrowActor = (options: CreateActorOptions): ActorSubclass<_ESCROW_SERVICE> =>
	createActor(escrowIdlFactory, { ...options, canisterId: PUBLIC_CANISTER_ID_ESCROW });

export const createLedgerActor = (options: CreateActorOptions): ActorSubclass<_LEDGER_SERVICE> =>
	createActor(ledgerIdlFactory, { ...options, canisterId: PUBLIC_CANISTER_ID_LEDGER });
