import { invalid, redirect } from '@sveltejs/kit';
import { defaultEvmStores, selectedAccount } from 'svelte-web3';

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
	if (selectedAccount) throw redirect(307, '/dashboard');
	return {
		user: locals.user && {
			address: selectedAccount
		}
	}
}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ cookies, request }) => {
		const evm = await defaultEvmStores.setProvider();

		const data = await request.formData();

		const body = {
			"address": selectedAccount
		};

		if (body.errors) {
			return invalid(401, body);
		}

		const value = btoa(JSON.stringify(body));

		cookies.set('jwt', value, { path: '/dashboard' });

		throw redirect(307, '/dashboard');
	}
};