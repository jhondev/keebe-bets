<script>
	import AuthButton from '$lib/components/auth/auth-button.svelte';
	import { app } from '$lib/store/auth';
	import { AppBar } from '@skeletonlabs/skeleton';
</script>

<AppBar>
	<svelte:fragment slot="lead">
		<a href="/" class="">
			<strong class="text-2xl pl-4">Keebe Bets</strong>
		</a>
	</svelte:fragment>
	<svelte:fragment slot="trail">
		<AuthButton>
			<h2 class="h3">
				<span title={$app.principal?.toString()} class="w-24 truncate text-ellipsis inline-block">
					{$app.principal}
				</span>
				{$app.balance} ICP
			</h2>
			<!-- {user ? user.name : ''} -->
		</AuthButton>
	</svelte:fragment>
</AppBar>
