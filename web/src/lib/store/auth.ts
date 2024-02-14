import type { _SERVICE as _ESCROW_SERVICE } from '$lib/canisters/declarations/escrow/escrow.did';
import type { _SERVICE as _LEDGER_SERVICE } from '$lib/canisters/declarations/ledger/ledger.did';
import type { ActorSubclass } from '@dfinity/agent';
import type { Principal } from '@dfinity/principal';
import { writable } from 'svelte/store';

export type AppStore = {
	loggedIn: boolean;
	principal: Principal | undefined;
	balance: number;
	escrow: ActorSubclass<_ESCROW_SERVICE> | undefined;
	ledger: ActorSubclass<_LEDGER_SERVICE> | undefined;
	refreshBalance: () => Promise<void>;
	logout: () => Promise<void>;
};

export const init = {
	loggedIn: false,
	principal: undefined,
	balance: 0,
	escrow: undefined,
	ledger: undefined,
	refreshBalance: async () => {},
	logout: async () => {}
};

export const app = writable<AppStore>(init);
