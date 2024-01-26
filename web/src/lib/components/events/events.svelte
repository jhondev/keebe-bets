<script lang="ts" context="module">
	export type Data = {
		game: Game;
		live: Event[];
		upcoming: Event[];
	};
</script>

<script lang="ts">
	import type { Event } from '$lib/apis/bets/events';
	import type { Game } from '$lib/apis/bets/games';

	export let data: Data;
	const { game, live, upcoming } = data;
</script>

<div class="flex flex-col">
	<h1>{game.name}</h1>
	<h2>Live</h2>
	<div>
		{#each live as event}
			<div>{event.name}</div>
		{/each}
	</div>
	<h2>Upcoming</h2>
	<div>
		{#each upcoming as event}
			<div>{event.name}</div>
		{/each}
	</div>
</div>
