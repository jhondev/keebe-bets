import { gamesEvents } from '$lib/apis/bets/events';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	return {
		gameEvts: gamesEvents
	};
};
