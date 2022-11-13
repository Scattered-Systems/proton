<script>
    import { info } from '$lib/constants'
    import { connected, defaultEvmStores, selectedAccount } from 'svelte-web3';

	async function handle_click() {
        if (!$connected) {
            defaultEvmStores.setProvider();
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

