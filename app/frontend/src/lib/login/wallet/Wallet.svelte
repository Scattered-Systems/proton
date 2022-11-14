<script>
    import { redirect } from '@sveltejs/kit';
    import { connected, defaultEvmStores, selectedAccount } from 'svelte-web3';

    import { info } from '$lib/constants'


	async function handle_click() {
        if (!$connected) {
            defaultEvmStores.setProvider();
            Response.redirect(307, '/dashboard');
        } else {
            defaultEvmStores.disconnect();
        }
    }
</script>

<button class="{info.theme.color.gradient.primary} {info.theme.button.layout.primary}" on:click={handle_click}>
    {#if !$connected}
        Connect
    {:else}
        {$selectedAccount}
    {/if}
</button>

<style>
	
</style>

