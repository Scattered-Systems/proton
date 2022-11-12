<script>
	/** @type {import('./$types').PageData} */
    export let data;
	import { connected, chainId, selectedAccount } from 'svelte-web3';
	import Text from '$lib/misc/Text.svelte';
    import Login from '$lib/login/Login.svelte';

	function load_chain_info() {
		if ($connected) {
			return {
				account: {
					address: $selectedAccount
				},
				chain: {
					id: $chainId
				}
			}
		}
	}
	
	$: props = data;
	$: eth = load_chain_info();
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="A powerful, cloud-native application" />
</svelte:head>

<section class="rounded bg-zinc-800 p-3">
	{#if $connected}
		<div class="flex flex-wrap items-center">
			<Text>{$eth.account.address}</Text>
		</div>
	{:else}
		<Login/>
	{/if}
	
</section>

<style>
	
</style>
