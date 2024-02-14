<script lang="ts">
	import type { Event } from '@prisma/client';
	import { Button } from '$lib/components/ui/button';
	import { socketStore } from '$lib/store/socket';
	import { onMount } from 'svelte';
	import type { Socket } from 'socket.io-client';
	import type { Bet } from '.';
	import { formatAmount } from '$lib/utils';
	import { Progress } from '$lib/components/ui/progress';

	export let event: Event;
	export let bet: Bet;

	let winner = '';
	let placingStatus = '';
	let placingPct = 0;

	let socket: Socket;

	onMount(() => {
		return socketStore.subscribe((value: Socket) => {
			socket = value;
		});
	});

	const takeBet = () => {
		socket.emit('bet-taken', bet);
	};
</script>

<div class="flex flex-col items-start gap-1 pb-5">
	<h1 class="text-2xl w-full text-center mb-5">
		Bet Winner: <span class="text-primary-900">{bet.winner}</span>
	</h1>
	<h1 class="text-yellow-500 text-5xl text-center w-full mb-3">{formatAmount(bet.amount)} ICP</h1>
	<div class="w-[90%] flex flex-row gap-2 text-2xl">
		<span>Winner:</span>
		<select class="select ml-2" bind:value={winner} placeholder="winner">
			<option value={event.teamA}>{event.teamA}</option>
			<option value={event.teamB}>{event.teamB}</option>
		</select>
	</div>
	<div class="text-lg text-center w-full pt-5">
		{#if placingStatus}
			<Progress value={placingPct} />
			<div class="mt-2">{placingStatus}</div>
		{:else}
			<Button on:click={takeBet} class="w-full text-xl">Take Bet</Button>
		{/if}
	</div>
</div>
