<script>
	import AuthButton from '$lib/components/auth/auth-button.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { app } from '$lib/store/auth';
	import { formatAmount } from '$lib/utils';
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
			<div class="flex gap-2 text-xl">
				<span title={$app.principal?.toString()} class="w-24 truncate text-ellipsis inline-block">
					{$app.principal}
				</span>
				<span>{formatAmount($app.balance)} ICP</span>
				<Button variant="secondary">Logout</Button>
			</div>
		</AuthButton>
	</svelte:fragment>
</AppBar>
