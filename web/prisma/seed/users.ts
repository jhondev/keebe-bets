import { PrismaClient } from '@prisma/client';

const db = new PrismaClient();

export const create = async () => {
	await db.user.create({
		data: {
			id: 'mabez-quy7w-nn6cn-xsk52-jqfu3-ulymn-kqqa2-mwgnj-2imz6-poxuz-vqe'
		}
	});

	await db.user.create({
		data: {
			id: '5qmhu-xsnod-ikal3-p44vm-bt5dc-ejcuz-usnjt-2pd46-xmv2r-xqmyr-cae'
		}
	});
};
