<script lang="ts">
	import AuthButton from '$lib/components/auth/auth-button.svelte';
	import { EventBets } from '$lib/components/bets';
	import Video from '$lib/components/ui/video.svelte';
	import { eventStore } from '$lib/store/event';

	$: event = $eventStore.selected;
	$: streamUrl = (() => {
		const url = event?.streamUrl ?? '/public/next-match-06.jpeg';
		return url.includes('jpeg') ? `/frames?src=${url}` : url;
	})();
</script>

{#if event}
	<Video src={streamUrl} />
	<h2 class="h3 text-center">{event.name}</h2>
	<h4 class="h4 text-center">
		{event.teamA}
		<span class="mx-3">{event.pointsA} : {event.pointsB}</span>
		{event.teamB}
	</h4>
	<hr class="mt-2 mb-5" />
	<AuthButton containerClass="text-center" text="Login for betting details">
		<EventBets {event} />
	</AuthButton>
{:else}
	<div class="flex items-center justify-center h-full">
		<h3 class="h3">Select a match for details</h3>
	</div>
{/if}
