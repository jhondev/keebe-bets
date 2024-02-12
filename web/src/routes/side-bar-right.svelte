<script lang="ts">
	import AuthButton from '$lib/components/auth/auth-button.svelte';
	import { PlaceBet } from '$lib/components/bets';
	import Video from '$lib/components/ui/video.svelte';
</script>

<Video videoId="fwAKNfEKV50" />
<hr />
<div>
	<h3 class="h3">Bets</h3>
	<AuthButton containerClass="mt-5 text-center">
		<PlaceBet />
	</AuthButton>
</div>
