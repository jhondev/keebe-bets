<script lang="ts">
	import { Events } from '$lib/components/events';

	export let data;
	const { gameEvts } = data;
</script>

<Events data={gameEvts} />
