<script lang="ts">
	import { PUBLIC_CANISTER_ID_IDENTITY, PUBLIC_DFX_NETWORK } from '$env/static/public';
	import { createEscrowActor, createLedgerActor } from '$lib/canisters/actors';
	import { app, init } from '$lib/store/auth';
	import { AuthClient } from '@dfinity/auth-client';
	import { onMount } from 'svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	export let text: string = 'Login';
	export let containerClass: string = '';

	let client: AuthClient;
	let loading = false;

	const getBalance = async () => {
		if (!$app.loggedIn || !$app.escrow) throw 'not logged in';
		const result = await $app.escrow.getCallerBalance();
		if ('Err' in result) throw 'Error: ' + result.Err[1];
		return result.Ok;
	};

	const handleAuth = async () => {
		const identity = client.getIdentity();
		app.set({
			loggedIn: true,
			principal: identity.getPrincipal(),
			balance: BigInt(0),
			escrow: createEscrowActor({ agentOptions: { identity } }),
			ledger: createLedgerActor({ agentOptions: { identity } }),
			refreshBalance: async () => {
				const balance = await getBalance();
				app.update((a) => ({ ...a, balance }));
			},
			logout: async () => {
				await client.logout();
				app.set(init);
			}
		});
		loading = true;
		await $app.refreshBalance();
		loading = false;
	};
	onMount(async () => {
		if (!client) {
			client = await AuthClient.create();
		}
		if (await client.isAuthenticated()) {
			handleAuth();
		}
	});

	function login() {
		client.login({
			identityProvider:
				PUBLIC_DFX_NETWORK === 'ic'
					? 'https://identity.ic0.app/#authorize'
					: `http://${PUBLIC_CANISTER_ID_IDENTITY}.localhost:4943`,
			onSuccess: handleAuth
		});
	}
</script>

{#if $app.loggedIn}
	{#if loading}
		Loading...
	{:else}
		<slot />
	{/if}
{:else}
	<div class={containerClass}>
		<Button class={$$props.class} on:click={login}>{text}</Button>
	</div>
{/if}
