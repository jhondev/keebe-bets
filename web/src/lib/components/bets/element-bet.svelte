<script lang="ts">
	import DialogBet from './dialog-bet.svelte';
	import { propousalBetsStore } from '$lib/store/gameInfoStore';
	import ProposeBet from './propose-bet.svelte';
	$: betAccepted = $propousalBetsStore.filter(bet => bet.status === true)
	$: betPending  = $propousalBetsStore.filter(bet => bet.status === false)

</script>

<ProposeBet />
<div class="max-w-md mx-auto p-3 border p-4 m-4 rounded bg-gray-900">
	<ul>
		{#each betAccepted as {user, amount}}
		<li class="flex">
			<div class="w-2/5">
				<h1>Accepted: $ {amount}</h1>
			</div>
			<div class="w-1/5">vs</div>
			<div class="w-2/5"><h1>{user.name} {user.lastname}</div>
		</li>
		{/each}
	</ul>
</div>

<div class="max-w-md mx-auto p-4">
	<ul>
		{#each betPending as { user, amount }}
			<li class="flex flex-row items-center mb-4">
				<div class="w-1/4">
					<img class="w-10 h-10 rounded-full mr-2" src={user.imageProfile} alt="Foto de usuario" />
				</div>
				<div class="w-1/2"><p class="font-bold">{`${user.name} ${user.lastname}`}</p></div>
				<div class="w-1/3">
					<h1>$ {amount}</h1>
				</div>
				<div class="w-1/3">
					<DialogBet propousalBet={{uid:user.id,amount}} />
				</div>
			</li>
		{/each}
	</ul>
</div>
