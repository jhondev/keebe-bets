import { balances } from '$lib/apis/users'
import type { Socket } from 'socket.io-client'
import { writable } from 'svelte/store';

export const dataGame = writable<number>(0);
export const dataBalances = writable<Record<string, number>>(balances);

export const socketStore = writable<Socket>();