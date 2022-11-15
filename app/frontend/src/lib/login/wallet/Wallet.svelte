<script>
    import { connected, defaultEvmStores, selectedAccount } from 'svelte-web3';

    import { info } from '$lib/constants'


	async function handle_click() {
        if (!$connected) {
            defaultEvmStores.setProvider();
        } else {
            defaultEvmStores.disconnect();
        }
    }
</script>

<div class="flex space-x-2 justify-center w-24">
    <button class="overflow-x-hidden {info.theme.color.gradient.primary} {info.theme.button.layout.primary}" on:click={handle_click}>
        {#if !$connected}
            Connect
        {:else}
            {$selectedAccount}
        {/if}
    </button>
</div>


<style>
	
</style>

