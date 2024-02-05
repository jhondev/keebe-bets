<script lang="ts">
	import { listPropousalBet } from '$lib/apis/bets/games';
	import DialogBet from './dialog-bet.svelte';
</script>

<div class="max-w-md mx-auto p-4">
	<ul>
		{#each listPropousalBet as { user, amount }}
			<li class="flex flex-row items-center mb-4">
				<div class="w-1/4">
					<img class="w-10 h-10 rounded-full mr-2" src={user.imageProfile} alt="Foto de usuario" />
				</div>
				<div class="w-1/2"><p class="font-bold">{`${user.name} ${user.lastname}`}</p></div>
				<div class="w-1/3">
					<h1>$ {amount}</h1>
				</div>
				<div class="w-1/3">
					<DialogBet />
				</div>
			</li>
		{/each}
	</ul>
</div>
