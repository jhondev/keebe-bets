<script lang="ts">
	import type { AccountIdentifier } from '$lib/canisters/declarations/ledger/ledger.did';
	import AuthButton from '$lib/components/auth-button.svelte';
	import { app } from '$lib/store/auth';

	let callerBalance = '0';
	let newBalance = '0';
	let betId = BigInt(0);
	let betBalance = '0';
	let canisterValue: string | undefined = '';
	let depositValue: string | undefined = '';
	let depositAddress: AccountIdentifier;

	const genUUID = () => {
		return BigInt(Math.floor(Date.now() + Math.random()));
	};

	const getCallerBalance = async () => {
		callerBalance = 'Loading...';
		const response = await $app.escrow?.get_caller_balance();
		callerBalance =
			response && 'Ok' in response ? response.Ok.toString() : 'Error: ' + response?.Err[1];
	};

	const getCanisterValue = async () => {
		canisterValue = 'Loading...';
		canisterValue = await $app.escrow?.get_value();
	};

	const placeBet = async () => {
		// betId = genUUID();
		// const response = await $auth.escrow?.place_bet(betId, BigInt(1 * 100000000));
		// console.log(response);
	};

	const getBetBalance = async () => {
		betBalance = 'Loading...';
		const response = await $app.escrow?.getPot(betId);
		betBalance =
			response && 'Ok' in response ? response.Ok.toString() : 'Error: ' + response?.Err[1];
	};

	const depositBet = async () => {
		if (!$app.escrow || !$app.ledger) return;
		betId = genUUID();
		depositValue = 'getting deposit address';
		depositAddress = await $app.escrow.get_deposit_address(betId);
		depositValue = 'Loading...';
		const rtransfer = await $app.ledger.transfer({
			memo: BigInt(0x1),
			amount: { e8s: BigInt(1 * 100000000) },
			fee: { e8s: BigInt(10000) },
			to: depositAddress,
			from_subaccount: [], // it will use default caller subaccount
			created_at_time: []
		});
		depositValue = 'Ok' in rtransfer ? rtransfer.Ok.toString() : 'Error: ' + rtransfer?.Err;
	};
</script>

<div class="container h-full mx-auto flex justify-center items-center">
	<div class="space-y-10 text-center flex flex-col items-center">
		<h2 class="h2">ICP BookMaker</h2>
		<div class="flex flex-col gap-3 justify-center space-x-2">
			<AuthButton>
				<h3 class="h3">Principal: {$app.principal}</h3>
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
				<hr />
				<button class="btn variant-filled-primary" on:click={depositBet}>Deposit Bet</button>
				<span>Deposit Address: {depositAddress}</span>
				<span>Bet {betId} Balance: {depositValue}</span>
			</AuthButton>
		</div>
	</div>
</div>
