<script lang="ts">
	import type { Event } from '@prisma/client';
	import * as Dialog from '$lib/components/ui/dialog';
	import { flyAndScale } from '$lib/utils';
	import { Button } from '../ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import PlaceBet from './place-bet.svelte';

	export let event: Event;
	let dialogOpen = false;
</script>

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Trigger class="btn variant-filled-primary w-full">Place Bet</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content
			transition={flyAndScale}
			class="fixed left-[50%] top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-50%] rounded-card-lg border bg-background p-5 shadow-popover outline-none sm:max-w-[490px] md:w-full"
		>
			<Dialog.Title
				class="flex w-full items-center justify-center text-lg font-semibold tracking-tight"
			>
				Propose Bet
			</Dialog.Title>
			<Separator class="-mx-5 mb-6 mt-5 block h-px bg-muted" />
			<!-- <Dialog.Description class="text-sm text-foreground-alt">
				Please confirm Bet
			</Dialog.Description> -->
			<PlaceBet {event} />
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
