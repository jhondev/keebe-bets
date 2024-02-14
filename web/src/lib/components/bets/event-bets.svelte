<script lang="ts">
	import type { Event } from '@prisma/client';
	import type { Bet } from '.';
	import PlaceBetDialog from './place-bet-dialog.svelte';
	import TakeBetDialog from './take-bet-dialog.svelte';
	import { app } from '$lib/store/auth';
	import { formatAmount } from '$lib/utils';
	import Separator from '../ui/separator/separator.svelte';

	export let event: Event;

	let bets: Bet[] = [];
	const userId = $app.principal?.toString();
	// Note: unoptimized code for demonstration purposes
	$: placedBets = bets.filter((b) => b.creatorId === userId);
	$: takenBets = bets.filter(
		(b) => b.creatorId !== userId && b.participants?.find((p) => p.participantId === userId)
	);
	$: openBets = bets.filter((b) => !b.participants?.find((p) => p.participantId === userId));

	$: loadBets(event.id);

	const loadBets = async (id: bigint) => {
		const response = await fetch(`/api/events/${id}/bets`);
		bets = await response.json();
		console.log(bets);
	};
</script>

<div class="mb-3">
	<PlaceBetDialog {event} />
</div>
{#if bets.length === 0}
	<div class="pt-16 text-center">
		<h3 class="h3">Still no bets placed for this event</h3>
	</div>
{:else}
	<div class="flex flex-col gap-2">
		<ul class="flex flex-col gap-1">
			{#each placedBets as bet}
				<li class="flex justify-between px-4 py-3 rounded-lg border border-primary-900">
					<h1>Placed Bet: {formatAmount(bet.amount)} ICP</h1>
					<h1>Participants: {bet.participants.length}</h1>
				</li>
			{/each}
		</ul>

		<ul class="flex flex-col gap-1">
			{#each takenBets as bet}
				<li class="flex justify-between px-4 py-3 rounded-lg border border-secondary-700">
					<h1>Taken Bet: {formatAmount(bet.amount)} ICP</h1>
					<p class="font-bold w-24 truncate text-ellipsis inline-block">{bet.creatorId}</p>
					<h1>Participants: {bet.participants.length}</h1>
				</li>
			{/each}
		</ul>
		<Separator />
		{#if openBets.length === 0}
			<h3 class="h3 text-center">No bets to take yet</h3>
		{:else}
			<h3 class="h3 text-center">Bets to take</h3>
		{/if}
		<ul class="flex flex-col gap-1">
			{#each openBets as bet}
				<li class="flex justify-between px-4 py-3 rounded-lg border border-tertiary-700">
					<h1>Bet: {formatAmount(bet.amount)} ICP</h1>
					<p class="font-bold w-24 truncate text-ellipsis inline-block">{bet.creatorId}</p>
					<!-- <div class="w-1/3"> -->
					<TakeBetDialog {bet} />
					<!-- </div> -->
				</li>
			{/each}
		</ul>
	</div>
{/if}
