export type Game = {
	slug: string;
	name: string;
	boxUrl: string;
	bannerUrl: string;
	avatarUrl: string;
};

export const games: Map<string, Game> = new Map([
	[
		'all',
		{
			slug: 'all',
			name: 'All Games',
			boxUrl: '/public/box-all.png',
			bannerUrl: '/public/banner-all.png',
			avatarUrl: '/public/avatar-all.png'
		}
	],
	[
		'lol',
		{
			slug: 'lol',
			name: 'League of Legends',
			boxUrl: '/public/box-lol.png',
			bannerUrl: '/public/banner-lol.png',
			avatarUrl: '/public/avatar-lol.png'
		}
	],
	[
		'dota2',
		{
			slug: 'dota2',
			name: 'Dota2',
			boxUrl: '/public/box-dota.png',
			bannerUrl: '/public/banner-dota.png',
			avatarUrl: '/public/avatar-dota2.png'
		}
	],
	[
		'csgo',
		{
			slug: 'csgo',
			name: 'CS:GO',
			boxUrl: '/public/box-csgo.png',
			bannerUrl: '/public/banner-csgo.png',
			avatarUrl: '/public/avatar-csgo.png'
		}
	],
	[
		'smite',
		{
			slug: 'smite',
			name: 'Smite',
			boxUrl: '/public/box-smite.png',
			bannerUrl: '/public/banner-smite.png',
			avatarUrl: '/public/avatar-smite.png'
		}
	],
	[
		'heartstone',
		{
			slug: 'heartstone',
			name: 'HeartStone',
			boxUrl: '/public/box-heartstone.png',
			bannerUrl: '/public/banner-heartstone.png',
			avatarUrl: '/public/avatar-heartstone.png'
		}
	],
	[
		'heroes',
		{
			slug: 'heroes',
			name: 'Heroes of the Storm',
			boxUrl: '/public/box-heroes.png',
			bannerUrl: '/public/banner-heroes.png',
			avatarUrl: '/public/avatar-heroes.png'
		}
	],
	[
		'startcraft',
		{
			slug: 'startcraft',
			name: 'StartCraft',
			boxUrl: '/public/box-startcraft.png',
			bannerUrl: '/public/banner-startcraft.png',
			avatarUrl: '/public/avatar-startcraft.png'
		}
	],
	[
		'worldoftanks',
		{
			slug: 'worldoftanks',
			name: 'World of Tanks',
			boxUrl: '/public/box-worldoftanks.png',
			bannerUrl: '/public/banner-worldoftanks.png',
			avatarUrl: '/public/avatar-worldoftanks.png'
		}
	],
	[
		'eafc',
		{
			slug: 'eafc',
			name: 'EA FC',
			boxUrl: '/public/box-eafc.png',
			bannerUrl: '/public/banner-eafc.png',
			avatarUrl: '/public/avatar-eafc.png'
		}
	]
]);

export const getGame = (slug: string) => {
	return (
		games.get(slug) ?? {
			slug: 'all',
			name: 'All Games',
			boxUrl: '/public/box-all.png',
			bannerUrl: '/public/banner-all.png',
			avatarUrl: '/public/avatar-all.png'
		}
	);
};
