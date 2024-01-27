<script lang="ts">
	import type { Game } from '$lib/apis/bets/games';

	export let games: Map<string, Game>;
</script>

<nav class="list-nav">
	<ul>
		{#each games as [_, game]}
			<li>
				<a href={`/games/${game.slug}`}>
					<img class="" loading="eager" alt="" src={game.boxUrl} />
				</a>
			</li>
		{/each}
	</ul>
</nav>
