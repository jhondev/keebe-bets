import { db } from '$lib/db';
import { json } from '@sveltejs/kit';

export async function GET({ params }) {
	// NOTE: unoptimized query for demonstration purposes
	const eventBets = await db.event.findUnique({
		select: {
			bets: {
				include: {
					participants: {
						include: {
							participant: true
						}
					}
				}
			}
		},
		where: { id: parseInt(params.id) }
	});
	return json(eventBets?.bets);
}
