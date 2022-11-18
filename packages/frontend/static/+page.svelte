<script>
    import { connected } from 'svelte-web3';
    import Login from '$lib/login/Login.svelte';
</script>

{#if !$connected }
    <Login/>
{:else}
    {Response.redirect("/dashboard", 200)}
{/if}

<style>
	
</style>

