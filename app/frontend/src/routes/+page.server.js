import { redirect } from '@sveltejs/kit';
import { connected, selectedAccount } from 'svelte-web3';

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
    if (connected) throw redirect(307, '/dashboard');
    
    return {
        user: locals.user && {
            email: locals.user.email,
            password: locals.user.password,
            username: locals.user.username,
            account: {
                address: selectedAccount
            }
        }
    };
};