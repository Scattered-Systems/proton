import { invalid, redirect } from '@sveltejs/kit';
import * as api from '$lib/api.js';
import { connected, defaultEvmStores, selectedAccount } from 'svelte-web3';
import * as dotenv from 'dotenv' // see https://github.com/motdotla/dotenv#how-do-i-use-dotenv-with-import
dotenv.config()

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
	if (locals.user) throw redirect(307, '/dashboard');

}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async ({ cookies, request }) => {
		const payload = {
			access_type: "offline",
			client_id: process.env.CLIENT_ID,
			redirect_uri: "http://localhost:5173",
			request_credentials: "default",
			response_type: "code",
			scope: "**"
			
		}

		const provider = async () => await defaultEvmStores.setProvider();

		const data = await request.formData();

		const body = {
			address: provider.selectedAccount
		};

		if (body.errors) {
			return invalid(401, body);
		}

		const value = btoa(JSON.stringify(body));
		cookies.set('jwt', value, { path: '/dashboard' });

		throw redirect(307, '/dashboard');
	}
};