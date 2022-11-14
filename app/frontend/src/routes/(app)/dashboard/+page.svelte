<script>
	import Section from '$lib/sections/Section.svelte';
	import Text from '$lib/misc/text/Text.svelte';
    import { chainId, connected, selectedAccount } from 'svelte-web3';

    async function load_chain_info() {
		return {
			account: {
				address: $selectedAccount
			},
			chain: {
				id: $chainId
			}
		}
	}
	
	$: eth = load_chain_info();
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="" />
</svelte:head>

<Section props="rounded bg-zinc-800 p-3 min-h-full">
    <div class="flex flex-wrap items-center">
        <Text>{$selectedAccount}</Text>
    </div>
</Section>