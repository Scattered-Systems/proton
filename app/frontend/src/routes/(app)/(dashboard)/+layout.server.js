import { selectedAccount } from 'svelte-web3';

/** @type {import('./$types').LayoutServerLoad} */
export function load({ locals }) {
	
	return {
		user: locals.user && {
			address: selectedAccount,
			email: locals.user.email,
			username: locals.user.username
		}
	};
}