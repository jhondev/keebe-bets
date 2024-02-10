<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Separator } from '$lib/components/ui/separator';
	import { flyAndScale } from '$lib/utils';
	import { Button } from '$lib/components/ui/button';
	import { dataBalances, socketStore } from '$lib/store/gameInfoStore';
	import { onDestroy, onMount } from 'svelte';
	import { users } from '$lib/apis/users';
	import type { IUser } from '$lib/models/user';
	import type { Socket } from 'socket.io-client';

	let dialogOpen = false;
	let user: IUser | null
	let balance : number =0
	let socketUser: Socket
	let unsub1: any
	
	export let propousalBet : {uid:String, amount:number}
	

	onMount(() =>{
		const params = new URLSearchParams(location.search)
		const idUser = params.get('user') ?? ''
		user = users[idUser]
		balance = 10

		unsub1 = socketStore.subscribe((value: Socket) => {
      		socketUser = value;
		});

		return unsub1

	})


	const acceptBet = () => {
		socketUser.emit('accept-bet', propousalBet);
		dialogOpen = false
	}

</script>

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Trigger >
		<Button >Accept Bet</Button>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content
			transition={flyAndScale}
			class="fixed left-[50%] top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-50%] rounded-card-lg border bg-background p-5 shadow-popover outline-none sm:max-w-[490px] md:w-full"
		>
			<Dialog.Title
				class="flex w-full items-center justify-center text-lg font-semibold tracking-tight"
				>Accept Bet</Dialog.Title
			>
			<Separator class="-mx-5 mb-6 mt-5 block h-px bg-muted" />
			<Dialog.Description class="text-sm text-foreground-alt">
				Please confirm Bet
			</Dialog.Description>
			<div class="flex flex-col items-start gap-1 pb-11 pt-7 items-center">
				<p class="text-sm font-medium text-center text-5xl">Amount</p>
				<h1 class="text-yellow-500 text-9xl">{propousalBet.amount}</h1>
				<div class="relative w-full">
				</div>
				<Button on:click={acceptBet} class="text-white py-2 px-4 rounded w-full text-3xl h-auto" >Accept Bet</Button>
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
			</div></Dialog.Content
		>
	</Dialog.Portal>
</Dialog.Root>
