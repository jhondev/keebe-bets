<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import { Separator } from '$lib/components/ui/separator';
	import { flyAndScale } from '$lib/utils';
	import { Button } from '$lib/components/ui/button';
	import TakeBet from './take-bet.svelte';
	import type { Bet } from '.';

	let dialogOpen = false;

	export let bet: Bet;
</script>

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Trigger>
		<Button>Accept Bet</Button>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content
			transition={flyAndScale}
			class="fixed left-[50%] top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-50%] rounded-card-lg border bg-background p-5 shadow-popover outline-none sm:max-w-[490px] md:w-full"
		>
			<Dialog.Title
				class="flex w-full items-center justify-center text-lg font-semibold tracking-tight"
			>
				Accept Bet
			</Dialog.Title>
			<Separator class="-mx-5 mb-6 mt-5 block h-px bg-muted" />
			<Dialog.Description class="text-sm text-foreground-alt">
				Please confirm Bet
			</Dialog.Description>
			<TakeBet {bet} />
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
