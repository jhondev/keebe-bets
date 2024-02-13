<script lang="ts">
	import AuthButton from '$lib/components/auth/auth-button.svelte';
	import { PlaceBet } from '$lib/components/bets';
	import Video from '$lib/components/ui/video.svelte';
	import { eventStore } from '$lib/store/event';

	$: match = $eventStore.selected;
	$: streamUrl = (() => {
		const url = match?.streamUrl ?? '/public/next-match-06.jpeg';
		return url.includes('jpeg') ? `/frames?src=${url}` : url;
	})();
</script>

{#if match}
	<Video src={streamUrl} />
	<h2 class="h3 text-center">{match.name}</h2>
	<h4 class="h4 text-center">
		{match.teamA}
		<span class="mx-3">{match.pointsA} : {match.pointsB}</span>
		{match.teamB}
	</h4>
	<hr class="mt-2 mb-5" />
	<AuthButton containerClass="text-center" text="Login for betting details">
		<PlaceBet />
	</AuthButton>
{:else}
	<div class="flex items-center justify-center h-full">
		<span class="h3">Select a match for details</span>
	</div>
{/if}
