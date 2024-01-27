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
	const { game, event } = data;
</script>

<div class="flex flex-row gap-1">
	<a class="" href={`/games/${game.slug}`}>
		<Avatar src={game.avatarUrl} width="w-14" rounded="rounded-l-md" />
	</a>
	<a class="logo-item px-1 py-0 w-0" href="#top">
		<span>{event.name}</span>
	</a>
	<a class="logo-item px-1 py-0" href="/">
		<span>{event.teamA.name}</span>
	</a>
	<a class="logo-item py-0 w-0" href="/">
		<span>{event.teamA.points}</span>
		<span>:</span>
		<span>{event.teamB.points}</span>
	</a>
	<a class="logo-item py-0 rounded-r-md" href="/">
		<span>{event.teamB.name}</span>
	</a>
</div>
