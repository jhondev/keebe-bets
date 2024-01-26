import { events } from '$lib/apis/bets/events';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	return {
		game: { slug: 'all' },
		events: {
			live: events.flatMap((e) => e.events.live),
			upcoming: events.flatMap((e) => e.events.upcoming)
		}
	};
};
