import { chainId, connected, selectedAccount } from 'svelte-web3';

/** @type {import('./$types').PageLoad} */
export async function load() {
    return {
        head: {
            description: '',
            title: 'Dashboard',
        },
        body: {},
        footer: {},
        data: [
            {
                id: 'ethereum',
                address: selectedAccount,
                chain_id: chainId,
                status: connected,

            }
        ]
    };
};