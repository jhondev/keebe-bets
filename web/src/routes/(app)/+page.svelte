<script lang="ts">
	import { Events } from '$lib/components/events';
	import { eventStore } from '$lib/store/event.js';
	import type { Event } from '@prisma/client';

	export let data;
	const { gameEvts } = data;

	const handleMatchClick = (e: CustomEvent<Event>) => {
		eventStore.set({ selected: e.detail });
	};
</script>

<div class="pl-28">
	<Events data={gameEvts} on:matchClick={handleMatchClick} />
</div>
