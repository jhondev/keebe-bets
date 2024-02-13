import { games } from '$lib/apis/bets/games';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async () => {
	return {
		games: games
	};
};
