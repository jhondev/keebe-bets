import Events from './events.svelte';
import Match from './match.svelte';

export type Event = {
	id: bigint;
	name: string;
	gameSlug: string;
	status: string;
	teamA: string;
	teamB: string;
	pointsA: number | null;
	pointsB: number | null;
	winner: string | null;
	startAt: Date;
	createdAt: Date;
};

export type GameEvents = {
	live: Event[];
	upcoming: Event[];
};

export { Events, Match };
