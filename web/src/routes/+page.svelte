<script lang="ts">
	import { games } from '$lib/apis/bets/games.js';
	import { Events } from '$lib/components/events';
	import { Button } from '$lib/components/ui/button';

	export let data;
	const { events } = data;
	const game = games.find((game) => game.slug === data.game.slug);
</script>

{#if !game}
	<div>Error finding game slug: {data.game.slug}</div>
{:else}
	<div class="container h-full mx-auto flex justify-center items-center">
		<div class="space-y-10 text-center flex flex-col items-center">
			<h2 class="h2">Welcome to Skeleton.</h2>

			<!-- Animated Logo -->
			<figure>
				<section class="img-bg" />
				<Events data={{ game: game, live: events.live, upcoming: events.upcoming }} />
			</figure>
			<Button>Place Bet</Button>
			<!-- / -->
		</div>
	</div>
{/if}
