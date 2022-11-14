import { selectedAccount } from 'svelte-web3';

/** @type {import('./$types').LayoutServerLoad} */
export function load({ locals }) {
	return {
		user: locals.user && {
			account: {
				address: selectedAccount
			},
			username: locals.user.username,
			email: locals.user.email,
			image: locals.user.image,
			bio: locals.user.bio,
		}
	};
}