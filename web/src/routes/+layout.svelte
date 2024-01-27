<script lang="ts">
	import '../app.postcss';
	import { AppShell, AppBar } from '@skeletonlabs/skeleton';
	import SideBarLeft from './side-bar-left.svelte';

	export let data;
	const { games } = data;
</script>

<AppShell
	slotSidebarLeft="bg-surface-500/5 w-56 p-4"
	slotPageContent="container flex flex-col h-full mx-auto px-5 py-4"
>
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
				<a href="/">
					<strong class="text-2xl pl-4">Keebe Bets</strong>
				</a>
			</svelte:fragment>
			<svelte:fragment slot="trail">John Doe</svelte:fragment>
		</AppBar>
	</svelte:fragment>

	<svelte:fragment slot="sidebarLeft">
		<SideBarLeft {games} />
	</svelte:fragment>

	<!-- Page Route Content -->
	<div class="gradient-bg"></div>
	<slot />
</AppShell>
