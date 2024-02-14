<script lang="ts">
	import type { Event } from '@prisma/client';
	import type { Socket } from 'socket.io-client';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { socketStore } from '$lib/store/socket';
	import { onMount } from 'svelte';
	import { app } from '$lib/store/auth';
	import { genUUID } from '$lib/utils';
	import { Progress } from '$lib/components/ui/progress';

	export let event: Event;

	let amount = 0;
	let winner = '';
	let placingStatus = '';
	let placingPct = 0;
	const userId = $app.principal?.toString();

	let socket: Socket;
	onMount(() => {
		return socketStore.subscribe((value: Socket) => {
			socket = value;
		});
	});

	const placeBet = async () => {
		const bet = {
			id: genUUID(),
			amount: BigInt(amount * 100000000),
			winner,
			creatorId: userId,
			eventId: event.id
		};
		// - emit bet-placed event
		// socket.emit('bet-placed', bet);
		// return;

		// steps
		// - prepare bet: save bet to db
		placingStatus = 'preparing bet';
		placingPct = 20;
		try {
			const response = await fetch(`/api/events/${event.id}/bets`, {
				method: 'POST',
				body: JSON.stringify(bet)
			});
			if (!response.ok) {
				placingStatus = 'Error: ' + response.statusText;
				return;
			}
		} catch (e) {
			placingStatus = 'Error: ' + e;
			return;
		}

		// - deposit icp: call ledger canister to make the bet deposit
		placingStatus = 'depositing icp';
		placingPct = 40;
		if (!$app.escrow || !$app.ledger) return;
		const depositAddress = await $app.escrow.getDepositAddress(bet.id);
		const tresult = await $app.ledger.transfer({
			memo: BigInt(0x1),
			amount: { e8s: BigInt(bet.amount) },
			fee: { e8s: BigInt(10000) },
			to: depositAddress,
			from_subaccount: [], // it will use default caller subaccount
			created_at_time: []
		});
		if ('Err' in tresult) {
			placingStatus = 'Error: ' + tresult.Err;
			return;
		}

		// - place bet: call escrow canister to place the bet in ICP
		placingStatus = 'placing bet';
		placingPct = 60;
		const result = await $app.escrow.placeBet(event.id, bet.id, bet.winner, bet.amount);
		if ('Err' in result) {
			placingStatus = 'Error: ' + result.Err[1];
			return;
		}

		// - update bet status: in db after escrow confirmation 'placed' (tx id?)
		// placingStatus = 'updating bet status';

		// - refresh balance: call ledger canister to refresh user balance
		placingStatus = 'refreshing balance';
		placingPct = 80;
		await $app.refreshBalance();
		placingStatus = 'Bet placed';
		placingPct = 100;

		// - emit bet-placed event
		socket.emit('bet-placed', bet);
	};
</script>

<div class="flex flex-col items-start gap-4 pb-5 pt-7">
	<div class="flex flex-row gap-2 text-2xl">
		<span>Amount:</span>
		<Input type="number" bind:value={amount} placeholder="Amount" class="h-10 text-2xl py-1" />
		<span>ICP</span>
	</div>
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
			<Button on:click={placeBet} class="w-full text-xl">Place Bet</Button>
		{/if}
	</div>
</div>
