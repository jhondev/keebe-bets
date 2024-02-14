<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { socketStore } from '$lib/store/gameInfoStore';
	import { onMount } from 'svelte';
	import { users } from '$lib/apis/users';
	import type { IUser } from '$lib/models/user';
	import type { Socket } from 'socket.io-client';
	import type { Bet } from '.';

	export let bet: Bet;

	let user: IUser | null;
	let balance: number = 0;
	let socketUser: Socket;
	let unsub1: any;

	onMount(() => {
		const params = new URLSearchParams(location.search);
		const idUser = params.get('user') ?? '';
		user = users[idUser];
		balance = 10;

		unsub1 = socketStore.subscribe((value: Socket) => {
			socketUser = value;
		});

		return unsub1;
	});

	const acceptBet = () => {
		socketUser.emit('accept-bet', bet);
	};
</script>

<div class="flex flex-col items-start gap-1 pb-11 pt-7">
	<p class="text-sm font-medium text-center">Amount</p>
	<h1 class="text-yellow-500 text-9xl">{bet.amount}</h1>
	<div class="relative w-full"></div>
	<Button on:click={acceptBet}>Accept Bet</Button>
	<!-- <div class="flex w-full justify-end">
					<Dialog.Close
						class="inline-flex h-input items-center justify-center rounded-input bg-dark px-[50px] text-[15px] font-semibold text-background shadow-mini hover:bg-dark/95 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-dark focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-98"
					>
						Save
					</Dialog.Close>
				</div>
				<Dialog.Close
					class="absolute right-5 top-5 rounded-md focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-foreground focus-visible:ring-offset-2 focus-visible:ring-offset-background active:scale-98"
				>
					<div>
						<span class="sr-only">Close</span>
					</div>
				</Dialog.Close> -->
</div>
