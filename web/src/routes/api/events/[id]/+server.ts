import { db } from '$lib/db';
import { json } from '@sveltejs/kit';

// endpoint to get event status
export async function GET({ params }) {
	const event = await db.event.findMany({
		select: {
			status: true
		},
		where: { id: parseInt(params.id) }
	});
	return json(event);
}
