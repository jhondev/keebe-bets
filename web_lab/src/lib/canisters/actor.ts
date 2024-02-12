import {
	Actor,
	HttpAgent,
	type ActorSubclass,
	type ActorMethod,
	type ActorConfig,
	type HttpAgentOptions,
	type Agent
} from '@dfinity/agent';
import { IDL } from '@dfinity/candid';
import type { Principal } from '@dfinity/principal';
import { PUBLIC_DFX_NETWORK, PUBLIC_DFX_HOST } from '$env/static/public';

declare interface CreateActorOptions {
	canisterId: string | Principal;
	agent?: Agent;
	agentOptions?: HttpAgentOptions;
	actorOptions?: ActorConfig;
}

export function createActor<T = Record<string, ActorMethod>>(
	interfaceFactory: IDL.InterfaceFactory,
	options: CreateActorOptions
): ActorSubclass<T> {
	const agent =
		options.agent ||
		new HttpAgent({
			host: PUBLIC_DFX_NETWORK === 'ic' ? `https://${options.canisterId}.ic0.app` : PUBLIC_DFX_HOST,
			...options.agentOptions
		});

	if (options?.agent && options.agentOptions) {
		console.warn(
			'Detected both agent and agentOptions passed to createActor. Ignoring agentOptions and proceeding with the provided agent.'
		);
	}

	// Fetch root key for certificate validation during development
	if (PUBLIC_DFX_NETWORK !== 'ic') {
		agent.fetchRootKey().catch((err) => {
			console.warn('Unable to fetch root key. Check to ensure that your local replica is running');
			console.error(err);
		});
	}

	return Actor.createActor(interfaceFactory, {
		canisterId: options.canisterId,
		agent,
		...options?.actorOptions
	});
}
