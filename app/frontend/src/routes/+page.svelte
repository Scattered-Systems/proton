<script>
    import { connected } from 'svelte-web3';
    import Login from '$lib/login/Login.svelte';
</script>

{#if !$connected }
    <Login/>
{/if}

<style>
	
</style>

