<script lang="ts">
	import { AppShell } from '@skeletonlabs/skeleton';
	import SideBarRight from './side-bar-right.svelte';
	import { onMount } from 'svelte';
	import { socketStore } from '$lib/store/socket';
	import { io } from 'socket.io-client';
	import Header from './header.svelte';

	// export let data;

	onMount(() => {
		const socket = io({
			// query: {
			// 	uid: idUser
			// }
		});
		socket.on('bet-placed', (bet) => {
			// propousalBetsStore.update((values) => {
			// 	const stateActual = [...values];
			// 	stateActual.push(proposal);
			// 	return stateActual;
			// });
		});
		socket.on('confirm-bet', (bet) => {
			// dataBalances.update((values) => {
			// 	const stateActual = Object.assign(values);
			// 	const balanceActualUser = [stateActual[message.uid]];
			// 	stateActual[message.uid] = Number(balanceActualUser) - message.amount;
			// 	return stateActual;
			// });
		});
		socketStore.set(socket);
		return () => {
			// unsubscribe();
		};
	});
</script>

<!-- slotSidebarLeft="w-56 p-4" -->
<AppShell
	slotSidebarRight="bg-surface-500/5 w-[30rem] p-4"
	slotPageContent="container flex flex-col h-full mx-auto px-5 py-4"
>
	<svelte:fragment slot="header">
		<Header />
	</svelte:fragment>

	<!-- <svelte:fragment slot="sidebarLeft">
		<SideBarLeft {games} />
	</svelte:fragment> -->
	<svelte:fragment slot="sidebarRight">
		<SideBarRight />
	</svelte:fragment>

	<!-- Page Route Content -->
	<div class="gradient-bg"></div>
	<slot />
</AppShell>
