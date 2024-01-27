export type Team = {
	name: string;
	points: number;
};
export type Event = {
	name: string;
	gameSlug: string;
	teamA: Team;
	teamB: Team;
};

export type GameEvents = {
	live: Event[];
	upcoming: Event[];
};

export const gamesEvents: GameEvents = {
	live: [
		{
			name: 'IEM Cologne',
			gameSlug: 'csgo',
			teamA: { name: 'Into The Breach', points: 2 },
			teamB: { name: 'Purple Haze', points: 3 }
		}
	],
	upcoming: []
};
