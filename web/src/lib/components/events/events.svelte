<script lang="ts">
	import type { Event } from '@prisma/client';
	import type { GameEvents } from '.';
	import { getGame } from '$lib/apis/bets/games';
	import Match from './match.svelte';
	import { createEventDispatcher } from 'svelte';

	export let data: GameEvents;
	const { live, upcoming } = data;
	const dispatch = createEventDispatcher();
	const selectedClass = 'variant-filled-secondary';

	let selected = { id: BigInt(0) };
	const matchClickEvent = (event: Event) => {
		selected = event;
		dispatch('matchClick', event);
	};
</script>

<div class="flex flex-col gap-6">
	<div>
		<h2 class="text-3xl mb-5">Live</h2>
		<div class="flex flex-col gap-2">
			{#each live as event}
				<Match
					class={selected.id === event.id ? selectedClass : ''}
					on:click={() => matchClickEvent(event)}
					data={{ game: getGame(event.gameSlug), event }}
				/>
			{/each}
		</div>
	</div>
	<div>
		<h2 class="text-3xl mb-5">Upcoming</h2>
		<div class="flex flex-col gap-2">
			{#each upcoming as event}
				<Match
					class={selected.id === event.id ? selectedClass : ''}
					on:click={() => matchClickEvent(event)}
					data={{ game: getGame(event.gameSlug), event }}
				/>
			{/each}
		</div>
	</div>
</div>
