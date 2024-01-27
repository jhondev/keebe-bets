<script lang="ts">
	import type { GameEvents } from '$lib/apis/bets/events';
	import { getGame } from '$lib/apis/bets/games';
	import Match from './match.svelte';

	export let data: GameEvents;
	const { live, upcoming } = data;
</script>

<div class="flex flex-col gap-10">
	<div class="flex flex-col gap-1">
		<h2 class="h2">Live</h2>
		{#each live as event}
			<Match data={{ game: getGame(event.gameSlug), event }} />
		{/each}
	</div>
	<h2 class="h2">Upcoming</h2>
	<div>
		{#each upcoming as event}
			<Match data={{ game: getGame(event.gameSlug), event }} />
		{/each}
	</div>
</div>
