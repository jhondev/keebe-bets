<script lang="ts">
	import type { Event } from '@prisma/client';
	import type { Socket } from 'socket.io-client';
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import { socketStore, userStorage } from '$lib/store/gameInfoStore';
	import { onMount } from 'svelte';
	import { users } from '$lib/apis/users';

	export let event: Event;

	let amountProposal = 0;
	$: idUser = $userStorage;

	let socketUser: Socket;
	onMount(() => {
		console.log(userStorage);

		const unsub1 = socketStore.subscribe((value: Socket) => {
			socketUser = value;
		});

		return unsub1;
	});

	const makeProposal = () => {
		console.log(amountProposal);
		const user = users[String(idUser)];
		console.log(event);
		const proposal = {
			user,
			amount: amountProposal,
			status: false
		};
		socketUser.emit('proposal-bet', proposal);
	};
</script>

<div class="flex flex-col items-start gap-1 pb-11 pt-7">
	<h1 class="text-3xl mb-5">Bet Amount</h1>
	<div class="relative w-full">
		<Input
			type="number"
			bind:value={amountProposal}
			placeholder="Amount"
			class="w-full text-center mb-3 text-3xl"
		/>
		<Button on:click={makeProposal} class="py-2 px-4 mb-2 rounded w-full text-2xl h-auto">
			Make Proposal
		</Button>
	</div>
</div>
