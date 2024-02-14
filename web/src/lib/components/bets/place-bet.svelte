<script lang="ts">
	import type { Bet, Event } from '@prisma/client';
	import type { Socket } from 'socket.io-client';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { socketStore } from '$lib/store/socket';
	import { onMount } from 'svelte';
	import { app } from '$lib/store/auth';
	import { genUUID } from '$lib/utils';

	export let event: Event;

	let amount = 0;
	$: userId = $app.principal?.toString();

	let socket: Socket;
	onMount(() => {
		return socketStore.subscribe((value: Socket) => {
			socket = value;
		});
	});

	const placeBet = async () => {
		const bet = {
			id: String(genUUID()),
			amount: amount * 100000000,
			creatorId: userId,
			eventId: String(event.id)
		};

		// steps
		// - prepare bet: save bet to db
		await fetch(`/api/events/${event.id}/bets`, {
			method: 'POST',
			body: JSON.stringify(bet)
		});
		// - call ledger canister to make the bet deposit
		// - call ledger canister to refresh user balance
		// - call escrow canister to place the bet in ICP
		// - update bet status in db after escrow confirmation 'placed' (tx id?)
		// - emit bet-placed event
		socket.emit('bet-placed', bet);
	};
</script>

<div class="flex flex-col items-start gap-3 pb-11 pt-7">
	<div class="flex flex-row gap-2 text-2xl mb-5 mx-auto">
		<h1 class="">Amount:</h1>
		<Input type="number" bind:value={amount} placeholder="Amount" class="w-40 h-9 text-2xl py-1" />
		<span>ICP</span>
	</div>
	<Button on:click={placeBet} class="w-full text-xl">Place Bet</Button>
</div>
