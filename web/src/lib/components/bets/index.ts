import EventBets from './event-bets.svelte';

export type Bet = {
	id: bigint;
	amount: number;
	creatorId: string;
	participants: { participantId: string }[];
};

export { EventBets };
