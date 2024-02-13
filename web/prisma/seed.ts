import { PrismaClient } from '@prisma/client';
import dayjs from 'dayjs';
import utc from 'dayjs/plugin/utc';
dayjs.extend(utc);

const db = new PrismaClient();
const genUUID = () => {
	return BigInt(Math.floor(Date.now() * Math.random()));
};

const now = dayjs().utc();
const start1 = now.add(1, 'hour').toDate();
const start2 = now.add(2, 'hour').toDate();

// *********************** Live ***********************
await db.event.create({
	data: {
		id: genUUID(),
		name: 'BetBoom Dacha Dubai',
		gameSlug: 'dota2',
		teamA: 'BetBoom Team',
		teamB: 'Gladiators',
		pointsA: '0',
		pointsB: '1',
		status: 'live',
		streamUrl: 'https://player.twitch.tv/?channel=one_passingby&parent=localhost',
		createdAt: now.toDate()
	}
});

await db.event.create({
	data: {
		id: genUUID(),
		name: 'HITPOINT MASTERS',
		gameSlug: 'lol',
		teamA: 'Team UNiTY',
		teamB: 'Dynamo Eclot',
		pointsA: '0',
		pointsB: '0',
		status: 'live',
		streamUrl: 'https://player.twitch.tv/?channel=uncorn85&parent=localhost',
		createdAt: now.toDate()
	}
});

// *********************** Upcoming ***********************
await db.event.create({
	data: {
		id: genUUID(),
		name: 'IEM Cologne',
		gameSlug: 'csgo',
		teamA: 'Into The Breach',
		teamB: 'Purple Haze',
		pointsA: '-',
		pointsB: '-',
		status: 'upcoming',
		startAt: start1,
		streamUrl: '/public/next-match-01.jpeg',
		createdAt: now.toDate()
	}
});

await db.event.create({
	data: {
		id: genUUID(),
		name: 'All Stars 2024',
		gameSlug: 'lol',
		teamA: 'NEVERWIN',
		teamB: 'Clover',
		pointsA: '-',
		pointsB: '-',
		status: 'upcoming',
		startAt: start1,
		streamUrl: '/public/next-match-02.jpeg',
		createdAt: now.toDate()
	}
});

await db.event.create({
	data: {
		id: genUUID(),
		name: 'BetBoom Dacha Dubai',
		gameSlug: 'dota2',
		teamA: 'LGD Gaming',
		teamB: 'Team Spirit',
		pointsA: '-',
		pointsB: '-',
		status: 'upcoming',
		startAt: start2,
		streamUrl: '/public/next-match-03.jpeg',
		createdAt: now.toDate()
	}
});

await db.event.create({
	data: {
		id: genUUID(),
		name: 'BetBoom Dacha Dubai',
		gameSlug: 'dota2',
		teamA: 'Xtreme Gaming',
		teamB: 'Team Spirit',
		pointsA: '-',
		pointsB: '-',
		status: 'upcoming',
		startAt: start2,
		streamUrl: '/public/next-match-04.jpeg',
		createdAt: now.toDate()
	}
});

await db.event.create({
	data: {
		id: genUUID(),
		name: 'Bundesliga Home Challenge',
		gameSlug: 'eafc',
		teamA: 'Paderborn',
		teamB: 'Arminia Bielefeld',
		pointsA: '-',
		pointsB: '-',
		status: 'upcoming',
		startAt: start2,
		streamUrl: '/public/next-match-05.jpeg',
		createdAt: now.toDate()
	}
});
