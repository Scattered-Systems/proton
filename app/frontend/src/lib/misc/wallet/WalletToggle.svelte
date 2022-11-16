<script>
    import { connected, defaultEvmStores, selectedAccount } from 'svelte-web3';

    import { theme } from '$lib/constants'


	async function handle_click() {
        if (!$connected) {
            defaultEvmStores.setProvider();
        } else {
            defaultEvmStores.disconnect();
        }
    }
</script>


<div class="flex flex-initial items-center justify-center mx-auto px-2">
    <button class="overflow-x-hidden py-1 px-2 {theme.button.primary} {theme.secondary}" on:click={handle_click}>
        {#if !$connected}
            Connect
        {:else}
            {$selectedAccount}
        {/if}
    </button>
</div>




<style>
	
</style>

