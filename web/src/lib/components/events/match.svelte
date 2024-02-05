<script lang="ts" context="module">
	import type { Game } from '$lib/apis/bets/games';
	import type { Event } from '$lib/apis/bets/events';
	import { Avatar } from '@skeletonlabs/skeleton';

	export type MatchEvent = {
		game: Game;
		event: Event;
	};
</script>

<script lang="ts">
	export let data: MatchEvent;
	import { dataGame } from '$lib/store/gameInfoStore';
	const { game, event } = data;

	const viewGameStatistics = (game: number) => dataGame.set(game);
</script>

<div class="flex flex-row gap-1 relative">
	<a class="logo-item w-1/12" href={`/games/${game.slug}`}>
		<Avatar src={game.avatarUrl} padding="" rounded="rounded-l-md" />
	</a>
	<a class="logo-item px-1 py-0 w-2/12" href="#top">
		<span>{event.name}</span>
	</a>
	<a class="logo-item px-1 py-0 w-3/12" href="/">
		<span>{event.teamA.name}</span>
	</a>
	<a class="logo-item py-0 w-2/12" href="/">
		<span>{event.teamA.points}</span>
		<span>:</span>
		<span>{event.teamB.points}</span>
	</a>
	<a class="logo-item py-0 rounded-r-md w-3/12" href="/">
		<span>{event.teamB.name}</span>
	</a>
	<span class="absolute top-0 right-0 inline-block">
		<span class="inline-block bg-red-500 text-white px-2 py-auto rounded-full animate-bounce">
			Live now
		</span>
	</span>
</div>
