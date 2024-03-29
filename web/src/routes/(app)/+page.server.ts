import { db } from '$lib/db';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	// Unoptimized query for demonstration purposes
	const gamesEvents = await db.event.findMany();
	return {
		gameEvts: {
			live: gamesEvents.filter((e) => e.status === 'live'),
			upcoming: gamesEvents.filter((e) => e.status === 'upcoming')
		}
	};
};
