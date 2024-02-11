<script lang="ts">
	import AuthButton from '$lib/components/auth-button.svelte';
	import { auth } from '$lib/store/auth';

	let callerBalance = '0';
	let newBalance = '0';
	let betId = BigInt(0);
	let betBalance = '0';
	let canisterValue: string | undefined = '';

	const genUUID = () => {
		return BigInt(Math.floor(Date.now() + Math.random()));
	};

	const getCallerBalance = async () => {
		callerBalance = 'Loading...';
		const response = await $auth.escrow?.get_caller_balance();
		callerBalance =
			response && 'Ok' in response ? response.Ok.toString() : 'Error: ' + response?.Err[1];
	};

	const getCanisterValue = async () => {
		canisterValue = 'Loading...';
		canisterValue = await $auth.escrow?.get_value();
	};

	const placeBet = async () => {
		betId = genUUID();
		const response = await $auth.escrow?.place_bet(betId, BigInt(1 * 100000000));
		console.log(response);
	};

	const getBetBalance = async () => {
		betBalance = 'Loading...';
		const response = await $auth.escrow?.getPot(betId);
		betBalance =
			response && 'Ok' in response ? response.Ok.toString() : 'Error: ' + response?.Err[1];
	};
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-10 text-center flex flex-col items-center">
		<h2 class="h2">ICP BookMaker</h2>
		<div class="flex flex-col gap-3 justify-center space-x-2">
			<AuthButton>
				<h3 class="h3">Principal: {$auth.principal}</h3>
				<hr />
				<button class="btn variant-filled-primary" on:click={getCallerBalance}>
					Caller Balance
				</button>
				<span>Balance: {callerBalance}</span>
				<hr />
				<button class="btn variant-filled-primary" on:click={getCanisterValue}>
					Get Canister Value
				</button>
				<span>Value: {canisterValue}</span>
				<hr />
				<button class="btn variant-filled-primary" on:click={placeBet}>Place Bet</button>
				<span>BetId: {betId}</span>
				<hr />
				<button class="btn variant-filled-primary" on:click={getBetBalance}>
					Get Bet Balance
				</button>
				<span>Bet {betId} Balance: {betBalance}</span>
			</AuthButton>
		</div>
	</div>
</div>
