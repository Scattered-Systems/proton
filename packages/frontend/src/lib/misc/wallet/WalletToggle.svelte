<script>
    import { connected, defaultEvmStores, selectedAccount } from 'svelte-web3';

    import { theme } from '$lib/constants'
    export let bg = theme.secondary;
    export let sz = {
        h: "",
        w: "w-24"
    }
	async function handle_click() {
        if (!$connected) {
            defaultEvmStores.setProvider();
        } else {
            defaultEvmStores.disconnect();
        }
    }
</script>


<button class="overflow-x-hidden py-1 px-2 inline-block mx-auto px-3 py-1 rounded items-center justify-center {bg} {sz.h} {sz.w}" on:click={handle_click}>
    {#if !$connected}
        Connect
    {:else}
        {$selectedAccount}
    {/if}
</button>




<style>
	
</style>

