<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { socketStore } from '$lib/store/socket';
	import { onMount } from 'svelte';
	import type { Socket } from 'socket.io-client';
	import type { Bet } from '.';
	import { formatAmount } from '$lib/utils';

	export let bet: Bet;

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

<div class="flex flex-col items-start gap-1 pb-11 pt-7">
	<p class="text-sm font-medium text-center">Amount</p>
	<h1 class="text-yellow-500 text-9xl">{formatAmount(bet.amount)}</h1>
	<div class="relative w-full"></div>
	<Button on:click={takeBet}>Accept Bet</Button>
</div>
