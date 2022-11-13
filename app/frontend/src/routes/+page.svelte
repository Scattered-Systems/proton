<script>
	import { connected, chainId, selectedAccount } from 'svelte-web3';
	import Login from '$lib/login/Login.svelte';

	async function handle() {
		if ($connected) {
			return Response.redirect('/dashboard', 200);
		}
	}
</script>

<svelte:head>
	<title>Proton</title>
	<meta name="description" content="A powerful, cloud-native application" />
</svelte:head>

{#if !($connected) }
	<Login/>
{/if}



<style>
	
</style>
