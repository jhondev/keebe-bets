// import type { Actions } from '@sveltejs/kit';
// import { createActor } from '$lib/canisters/declarations/escrow';

// export const actions: Actions = {
// 	async callerBalance({ params }) {
// 		console.log('getting caller balance');
// 		const escrow = createActor('br5f7-7uaaa-aaaaa-qaaca-cai', {
// 			agentOptions: { host: 'http://localhost:35753' }
// 		});
// 		const response = await escrow.get_caller_balance();
// 		const r = 'Ok' in response ? response.Ok.toString() : 'Error: ' + response.Err[1];
// 		console.log('callerBalance', r);
// 		return r;
// 	},

// 	async depositICP() {
// 		console.log('depositing ICP');
// 		const escrow = createActor('br5f7-7uaaa-aaaaa-qaaca-cai', {
// 			agentOptions: { host: 'http://localhost:35753' }
// 		});
// 		const response = await escrow.deposit_icp();
// 		const r = 'Ok' in response ? response.Ok.toString() : 'Error: ' + response.Err[1];
// 		console.log('deposit response', r);
// 		return r;
// 	},

// 	async getValue() {
// 		console.log('getting value');
// 		const escrow = createActor('br5f7-7uaaa-aaaaa-qaaca-cai', {
// 			agentOptions: { host: 'http://localhost:35753' }
// 		});
// 		const response = await escrow.get_value();
// 		console.log('getValue response', response);
// 		return response;
// 	}
// };
