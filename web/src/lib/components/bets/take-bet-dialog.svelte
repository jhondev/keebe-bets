<script lang="ts">
	import type { Event } from '@prisma/client';
	import type { Bet } from '.';
	import * as Dialog from '$lib/components/ui/dialog';
	import { Separator } from '$lib/components/ui/separator';
	import { flyAndScale } from '$lib/components/ui/utils';
	import { Button } from '$lib/components/ui/button';
	import TakeBet from './take-bet.svelte';

	let dialogOpen = false;

	export let event: Event;
	export let bet: Bet;
</script>

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Trigger>
		<Button>Take Bet</Button>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Content
			transition={flyAndScale}
			class="fixed left-[50%] top-[50%] z-50 w-full max-w-[94%] translate-x-[-50%] translate-y-[-50%] rounded-card-lg border bg-background p-5 shadow-popover outline-none sm:max-w-[490px] md:w-full"
		>
			<Dialog.Title class="text-center text-2xl">{event.name}</Dialog.Title>
			<Separator />
			<TakeBet {event} {bet} />
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
