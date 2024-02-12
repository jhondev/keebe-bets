<script lang="ts">
	import { PUBLIC_CANISTER_ID_IDENTITY, PUBLIC_DFX_NETWORK } from '$env/static/public';
	import { createEscrowActor, createLedgerActor } from '$lib/canisters/actors';
	import { app } from '$lib/store/auth';
	import { AuthClient } from '@dfinity/auth-client';
	import { onMount } from 'svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	let client: AuthClient;

	const handleAuth = async () => {
		const identity = client.getIdentity();
		app.update(() => ({
			loggedIn: true,
			principal: identity.getPrincipal(),
			escrow: createEscrowActor({ agentOptions: { identity } }),
			ledger: createLedgerActor({ agentOptions: { identity } }),
			logout: async () => {
				await client.logout();
				app.update((a) => ({ ...a, loggedIn: false, principal: undefined, escrow: undefined }));
			}
		}));
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
	<slot />
{:else}
	<Button on:click={login}>Login</Button>
{/if}
