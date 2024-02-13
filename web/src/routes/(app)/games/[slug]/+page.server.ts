import { gamesEvents } from '$lib/apis/bets/events';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const { slug } = params;
	return {
		gameEvts: {
			live: slug === 'all' ? gamesEvents.live : gamesEvents.live.filter((e) => e.gameSlug === slug),
			upcoming:
				slug === 'all'
					? gamesEvents.upcoming
					: gamesEvents.upcoming.filter((e) => e.gameSlug === slug)
		}
	};
};
