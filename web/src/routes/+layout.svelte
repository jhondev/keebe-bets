<script>
	import { AppShell } from '@skeletonlabs/skeleton';

	import '../app.pcss';
	import SideBarLeft from './side-bar-left.svelte';
	import Footer from './footer.svelte';
</script>

<AppShell
	class=""
	slotSidebarLeft="h-full p-3"
	slotPageContent="px-5 py-4"
	slotSidebarRight="h-full p-3"
>
	<svelte:fragment slot="header">
		<div class="flex flex-row gap-10">
			<div>logo</div>
			<ul class="flex flex-row gap-10">
				<li><a href="/">Home</a></li>
				<li><a href="/bets1">Bets1</a></li>
				<li><a href="/bets2">Bets2</a></li>
				<li><a href="/bets3">Bets3</a></li>
				<li><a href="/search">Search</a></li>
			</ul>
		</div>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft"><SideBarLeft /></svelte:fragment>
	<svelte:fragment slot="sidebarRight"></svelte:fragment>

	<slot />

	<svelte:fragment slot="footer"><Footer /></svelte:fragment>
</AppShell>
