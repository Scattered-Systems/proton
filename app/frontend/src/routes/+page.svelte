<script>
	import { connected, defaultEvmStores, chainId, selectedAccount } from 'svelte-web3';
	import Text from '$lib/misc/Text.svelte';
	function handle_connection() {
		if (connected) {
			return chainId
		} else {
			return false
		}
	}
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="A powerful, cloud-native application" />
</svelte:head>

<section class="rounded ">
	{#if $connected}
		<div class="flex flex-wrap items-center">
			<Text>Chain Id: {$chainId}</Text>
			<Text>Selected Account: {$selectedAccount}</Text>
		</div>
	{:else}
		<div class="flex grow items-center w-full">
			<Text>
				Login to continue...
			</Text>
		</div>
	{/if}
	
</section>

<style>
	
</style>
