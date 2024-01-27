<script lang="ts">
	import Events from '$lib/components/events/events.svelte';

	export let data;
	const { gameEvts } = data;
</script>

<Events data={gameEvts} />
