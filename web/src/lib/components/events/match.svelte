<script lang="ts" context="module">
	import type { Game } from '$lib/apis/bets/games';
	import type { Event } from '.';
	import { Avatar } from '@skeletonlabs/skeleton';

	export type MatchEvent = {
		game: Game;
		event: Event;
	};
</script>

<script lang="ts">
	export let data: MatchEvent;
	import { dataGame } from '$lib/store/gameInfoStore';
	import dayjs from 'dayjs';
	const { game, event } = data;

	const viewGameStatistics = (game: number) => dataGame.set(game);
</script>

<div class="flex flex-row gap-1 relative">
	<a class="w-1/12" href={`/games/${game.slug}`}>
		<Avatar src={game.avatarUrl} padding="" rounded="rounded-l-md" />
	</a>
	<a class="px-1 py-0 w-2/12" href="#top">
		<span>{event.name}</span>
	</a>
	<a class="px-1 py-0 w-3/12 text-right" href="/">
		<span>{event.teamA}</span>
	</a>
	<a class="py-0 w-2/12 flex flex-col gap-0 items-center" href="/">
		{#if event.status === 'upcoming'}
			<span class="mb-1">{dayjs(event.startAt).format('HH:00')}</span>
		{/if}
		<div class="flex text-center">
			<span>{event.pointsA}</span>
			<span class="mx-2">:</span>
			<span>{event.pointsB}</span>
		</div>
	</a>
	<a class="py-0 rounded-r-md w-3/12" href="/">
		<span>{event.teamB}</span>
	</a>
	{#if event.status === 'live'}
		<span class="absolute top-0 right-0 inline-block">
			<span class="inline-block bg-red-500 text-white px-2 py-auto rounded-full animate-bounce">
				Live now
			</span>
		</span>
	{/if}
</div>
