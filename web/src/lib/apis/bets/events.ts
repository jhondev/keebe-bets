export type Event = {
	name: string;
	contenderA: string;
	contenderB: string;
};
export const events = [
	{ game: { slug: 'lol' }, events: { live: [], upcoming: [] } },
	{ game: { slug: 'dota' }, events: { live: [], upcoming: [] } },
	{ game: { slug: 'csgo' }, events: { live: [], upcoming: [] } },
	{ game: { slug: 'smite' }, events: { live: [], upcoming: [] } }
];
