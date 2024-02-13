import { PrismaClient } from '@prisma/client';

const db = new PrismaClient();

const genUUID = () => {
	return BigInt(Math.floor(Date.now() + Math.random()));
};
const now = new Date();

await db.event.create({
	data: {
		id: genUUID(),
		name: 'IEM Cologne',
		gameSlug: 'csgo',
		teamA: 'Into The Breach',
		teamB: 'Purple Haze',
		status: 'upcoming',
		createdAt: now
	}
});
