import type { _SERVICE as _ESCROW_SERVICE } from '$lib/canisters/declarations/escrow/escrow.did';
import type { _SERVICE as _LEDGER_SERVICE } from '$lib/canisters/declarations/ledger/ledger.did';
import type { ActorSubclass } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import { writable } from 'svelte/store';

export type Appl = {
	loggedIn: boolean;
	principal: Principal | undefined;
	escrow: ActorSubclass<_ESCROW_SERVICE> | undefined;
	ledger: ActorSubclass<_LEDGER_SERVICE> | undefined;
	logout: () => Promise<void>;
};

export const app = writable<Appl>({
	loggedIn: false,
	principal: undefined,
	escrow: undefined,
	ledger: undefined,
	logout: async () => {}
});
