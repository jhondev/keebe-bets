<script lang="ts" context="module">
	import type { Game } from '$lib/apis/bets/games';
	import type { Event } from '@prisma/client';
	import { Avatar } from '@skeletonlabs/skeleton';

	export type MatchEvent = {
		game: Game;
		event: Event;
	};
</script>

<script lang="ts">
	import dayjs from 'dayjs';

	export let data: MatchEvent;
	const { game, event } = data;
</script>

<button
	on:click
	class={`btn variant-outline-secondary flex flex-row gap-1 relative ${$$props.class}`}
>
	<Avatar width="w-10" src={game.avatarUrl} rounded="rounded-l-md" />
	<div class="px-1 py-0 w-2/12">
		<span>{event.name}</span>
	</div>
	<div class="px-1 py-0 w-3/12 text-right">
		<span>{event.teamA}</span>
	</div>
	<div class="py-0 w-2/12 flex flex-col gap-0 items-center border-x-2">
		{#if event.status === 'upcoming'}
			<span class="mb-1">{dayjs(event.startAt).format('HH:00')}</span>
		{/if}
		<div class="flex text-center">
			<span>{event.pointsA}</span>
			<span class="mx-2">:</span>
			<span>{event.pointsB}</span>
		</div>
	</div>
	<div class="py-0 rounded-r-md w-3/12">
		<span>{event.teamB}</span>
	</div>
	{#if event.status === 'live'}
		<span class="absolute top-0 right-0 inline-block">
			<span class="inline-block bg-primary px-2 rounded-full animate-bounce"> Live </span>
		</span>
	{/if}
</button>
