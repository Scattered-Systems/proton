import { redirect } from 'svelte';
import { chainId, selectedAccount } from 'svelte-web3';

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
    if (!selectedAccount) throw redirect(306, '/login');
    return {
        user: locals.user && {
            address: selectedAccount
        }
    };
};