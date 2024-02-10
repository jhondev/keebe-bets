<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
    import { flyAndScale } from '$lib/utils';
	import { Button } from "../ui/button";
    import { Separator } from '$lib/components/ui/separator';
	import { Input } from '../ui/input';
	import { users } from '$lib/apis/users';
	import { socketStore, userStorage } from '$lib/store/gameInfoStore';
	import type { Socket } from 'socket.io-client';
	import { onMount } from 'svelte';

    let dialogOpen = false;
	$: idUser = $userStorage
	let amountProposal = 0
	let socketUser: Socket 
	onMount(() => {
		console.log(userStorage);

		const unsub1 = socketStore.subscribe((value: Socket) => {
      		socketUser = value;
		});

		return unsub1
	})

    const makeProposal = () => {
		console.log(amountProposal);
		const user = users[String(idUser)]
		console.log(user);
		const proposal = {
			user,
			amount: amountProposal,
			status: false
		}
		socketUser.emit('proposal-bet',proposal)
	}
</script>


<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Trigger >
		<Button variant="destructive" class="mt-2">Propose a bet</Button>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content
			transition={flyAndScale}
			class="fixed left-[50%] top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-50%] rounded-card-lg border bg-background p-5 shadow-popover outline-none sm:max-w-[490px] md:w-full"
		>
			<Dialog.Title
				class="flex w-full items-center justify-center text-lg font-semibold tracking-tight"
				>Propose Bet</Dialog.Title
			>
			<Separator class="-mx-5 mb-6 mt-5 block h-px bg-muted" />
			<!-- <Dialog.Description class="text-sm text-foreground-alt">
				Please confirm Bet
			</Dialog.Description> -->
			<div class="flex flex-col items-start gap-1 pb-11 pt-7 items-center">
				<h1 class="text-yellow-500 text-3xl mb-5">Proposal amount</h1>
				<div class="relative w-full ">
                    <Input type="number" bind:value={amountProposal} placeholder="Amount" class="w-full text-center mb-3 text-3xl"  />
                    <Button on:click={makeProposal} class="text-white py-2 px-4 mb-2 rounded w-full text-2xl h-auto" >Make Proposal</Button>
				</div>
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
