import type { Event } from '@prisma/client';
import { writable } from 'svelte/store';

export type EventStore = {
	selected: Event | undefined;
};

export const eventStore = writable<EventStore>({
	selected: undefined
});
