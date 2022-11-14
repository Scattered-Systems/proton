import { redirect } from '@sveltejs/kit';
import { connected, selectedAccount } from 'svelte-web3';

/** @type {import('./$types').LayoutServerLoad} */
export function load({ locals }) {
    if (locals.user) throw redirect(307, '/dashboard');
    return {
        user: locals.user && {
            account: {
                address: selectedAccount
            }
        }
    }
}
