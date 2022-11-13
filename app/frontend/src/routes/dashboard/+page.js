
import {connected} from 'svelte-web3';

/** @type {import('./$types').PageLoad} */
export async function load() {
    let user = {
        auth: connected
    }
    return {
        user
    };
};