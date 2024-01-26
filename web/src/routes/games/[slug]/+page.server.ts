import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	return {
		game: {
			name: params.slug,
			slug: params.slug
		}
	};
};
