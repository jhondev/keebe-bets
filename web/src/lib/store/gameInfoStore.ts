import { listPropousalBet } from '$lib/apis/bets/games';
import { balances } from '$lib/apis/users';
import type { PropousalBet } from '$lib/models/interfaces';
import type { Socket } from 'socket.io-client';
import { writable } from 'svelte/store';

export const dataGame = writable<number>(0);
export const dataBalances = writable<Record<string, number>>(balances);
export const propousalBetsStore = writable<PropousalBet[]>(listPropousalBet);
export const socketStore = writable<Socket>();
export const userStorage = writable<string>();
