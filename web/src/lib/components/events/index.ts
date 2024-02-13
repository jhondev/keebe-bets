import type { Event } from '@prisma/client';
import Events from './events.svelte';
import Match from './match.svelte';

export type GameEvents = {
	live: Event[];
	upcoming: Event[];
};

export { Events, Match };
