import EventBets from './event-bets.svelte';

export type Bet = {
	id: bigint;
	amount: number;
	winner: string;
	creatorId: string;
	participants: { participantId: string }[];
};

export { EventBets };
