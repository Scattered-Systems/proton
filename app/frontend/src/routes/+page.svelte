<script>
	import { connected, chainId, selectedAccount } from 'svelte-web3';
	import Login from '$lib/login/Login.svelte';
	import Text from '$lib/misc/Text.svelte';

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
	
	$: eth = load_chain_info();
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="A powerful, cloud-native application" />
</svelte:head>

{#if !($connected) }
	<Login/>
{:else}
	<section class="rounded bg-zinc-800 p-3 min-h-full">
		<div class="flex flex-wrap items-center">
			<Text>{$eth.account.address}</Text>
		</div>
	</section>
{/if}



<style>
	
</style>
