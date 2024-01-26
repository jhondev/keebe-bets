<script lang="ts">
	import type { Game } from '$lib/apis/bets/games';

	export let games: Game[];
</script>

<nav class="list-nav">
	<ul>
		{#each games as game}
			<li>
				<a href={'/games/' + game.slug}>
					<img class="" loading="eager" alt="" src={game.imgUrl} />
				</a>
			</li>
		{/each}
	</ul>
</nav>
