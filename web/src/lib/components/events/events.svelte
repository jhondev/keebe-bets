<script lang="ts">
	import type { GameEvents } from '.';
	import { getGame } from '$lib/apis/bets/games';
	import Match from './match.svelte';

	export let data: GameEvents;
	const { live, upcoming } = data;
</script>

<div class="flex flex-col gap-10">
	<div>
		<h2 class="h2 mb-5">Live</h2>
		<div class="flex flex-col gap-4">
			{#each live as event}
				<Match data={{ game: getGame(event.gameSlug), event }} />
			{/each}
		</div>
	</div>
	<div>
		<h2 class="h2 mb-5">Upcoming</h2>
		<div class="flex flex-col gap-4">
			{#each upcoming as event}
				<Match data={{ game: getGame(event.gameSlug), event }} />
			{/each}
		</div>
	</div>
</div>
