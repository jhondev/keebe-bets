export type Game = {
	slug: string;
	name: string;
	imgUrl: string;
};
export const games: Game[] = [
	{ slug: 'all', name: 'All Games', imgUrl: '/public/box-all.png' },
	{ slug: 'lol', name: 'League of Legends', imgUrl: '/public/box-lol.png' },
	{ slug: 'dota', name: 'Dota', imgUrl: '/public/box-dota.png' },
	{ slug: 'csgo', name: 'CS:GO', imgUrl: '/public/box-csgo.png' },
	{ slug: 'smite', name: 'Smite', imgUrl: '/public/box-smite.png' },
	{ slug: 'heartstone', name: 'HeartStone', imgUrl: '/public/box-heartstone.png' },
	{ slug: 'heroes', name: 'Heroes of the Storm', imgUrl: '/public/box-heroes.png' },
	{ slug: 'startcraft', name: 'StartCraft', imgUrl: '/public/box-startcraft.png' },
	{ slug: 'worldoftanks', name: 'World of Tanks', imgUrl: '/public/box-worldoftanks.png' }
];
