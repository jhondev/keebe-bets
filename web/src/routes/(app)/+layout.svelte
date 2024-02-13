<script lang="ts">
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import SideBarLeft from './side-bar-left.svelte';
	import SideBarRight from './side-bar-right.svelte';
	import { onMount } from 'svelte';
	import { users } from '$lib/apis/users';
	import type { IUser } from '$lib/models/user';
	import {
		dataBalances,
		propousalBetsStore,
		socketStore,
		userStorage
	} from '$lib/store/gameInfoStore';
	import { io } from 'socket.io-client';
	import AuthButton from '$lib/components/auth/auth-button.svelte';

	export let data;
	let user: IUser | null = null;
	let balance: number = 0;
	const { games } = data;
	$: balans = $dataBalances;

	onMount(() => {
		const params = new URLSearchParams(location.search);
		const idUser = params.get('user') ?? '';
		user = users[idUser];
		userStorage.set(idUser);

		const unsubscribe = dataBalances.subscribe((value) => {
			balance = value[idUser];
		});
		const socket = io({
			query: {
				uid: idUser
			}
		});
		socket.on('eventFromServer', (message) => {
			console.log(message);
		});
		socket.on('confirm-bet', (message) => {
			dataBalances.update((values) => {
				const stateActual = Object.assign(values);
				const balanceActualUser = [stateActual[message.uid]];
				stateActual[message.uid] = Number(balanceActualUser) - message.amount;
				return stateActual;
			});
		});
		socket.on('proposal-bet', (proposal) => {
			propousalBetsStore.update((values) => {
				const stateActual = [...values];
				stateActual.push(proposal);
				return stateActual;
			});
		});
		socketStore.set(socket);
		return () => {
			unsubscribe();
		};
	});
</script>

<!-- slotSidebarLeft="w-56 p-4" -->
<AppShell
	slotSidebarRight="bg-surface-500/5 w-[30rem] p-4"
	slotPageContent="container flex flex-col h-full mx-auto px-5 py-4"
>
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
				<a href="/" class="">
					<strong class="text-2xl pl-4">Keebe Bets</strong>
				</a>
			</svelte:fragment>
			<svelte:fragment slot="trail">
				<AuthButton>
					<h2 class="text-yellow-600 text-3xl pe-4">Balance: $ {balance}</h2>
					{user ? user.name : ''}
				</AuthButton>
			</svelte:fragment>
		</AppBar>
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
