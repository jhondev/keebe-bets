<script lang="ts">
	import type { Event } from '@prisma/client';
	import type { Bet } from '.';
	import PlaceBetDialog from './place-bet-dialog.svelte';
	import TakeBetDialog from './take-bet-dialog.svelte';
	import { onMount } from 'svelte';
	import { app } from '$lib/store/auth';

	export let event: Event;

	let bets: Bet[] = [];
	const userId = $app.principal?.toString();
	// Note: unoptimized code for demonstration purposes
	$: placedBets = bets.filter((b) => b.creatorId === userId);
	$: takenBets = bets.filter(
		(b) => b.creatorId !== userId && b.participants?.find((p) => p.id === userId)
	);
	$: openBets = bets.filter((b) => !b.participants?.find((p) => p.id === userId));

	onMount(async () => {
		const response = await fetch(`/api/events/${event.id}/bets`);
		bets = await response.json();
		console.log(bets);
	});
</script>

<PlaceBetDialog {event} />
{#if bets.length === 0}
	<div class="pt-16 text-center">
		<h3 class="h3">Still no bets placed for this event</h3>
	</div>
{:else}
	<div class="max-w-md mx-auto p-3 border m-4 rounded">
		<ul>
			{#each placedBets as bet}
				<li class="flex">
					<div class="w-2/5">
						<h1>{bet.amount} ICP</h1>
					</div>
					<div class="w-1/5">vs</div>
					<div class="w-2/5"><h1>Players: {bet.participants.length}</h1></div>
				</li>
			{/each}
		</ul>
	</div>

	<div class="max-w-md mx-auto p-3 border m-4 rounded">
		<ul>
			{#each takenBets as bet}
				<li class="flex">
					<div class="w-2/5">
						<h1>{bet.amount} ICP</h1>
					</div>
					<div class="w-1/5">vs</div>
					<div class="w-2/5"><h1>Players: {bet.participants.length}</h1></div>
				</li>
			{/each}
		</ul>
	</div>

	<div class="max-w-md mx-auto p-4">
		<ul>
			{#each openBets as bet}
				<li class="flex flex-row items-center mb-4">
					<div class="w-1/2">
						<p class="font-bold w-24 truncate text-ellipsis inline-block">{bet.creatorId}</p>
					</div>
					<div class="w-1/3">
						<h1>{bet.amount} ICP</h1>
					</div>
					<div class="w-1/3">
						<TakeBetDialog {bet} />
					</div>
				</li>
			{/each}
		</ul>
	</div>
{/if}
