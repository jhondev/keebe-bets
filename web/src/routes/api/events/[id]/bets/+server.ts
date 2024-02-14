import { db } from '$lib/db';
import type { Bet } from '@prisma/client';
import { json } from '@sveltejs/kit';

export async function GET({ params }) {
	// NOTE: unoptimized query for demonstration purposes
	const eventBets = await db.event.findUnique({
		select: {
			bets: {
				include: {
					participants: true
				}
			}
		},
		where: { id: parseInt(params.id) }
	});
	return json(eventBets?.bets);
}

export async function POST({ request }) {
	const bet: Bet = await request.json();
	console.log(bet);
	await db.bet.create({
		data: {
			...bet,
			participants: {
				create: {
					participantId: bet.creatorId
				}
			}
		}
	});

	return json({});
}
