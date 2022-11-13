<script>
	import Text from '$lib/misc/Text.svelte';
    /** @type {import('./$types').PageData} */
    export let data;

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
	<meta name="description" content="" />
</svelte:head>

<section class="rounded bg-zinc-800 p-3 min-h-full">
    <div class="flex flex-wrap items-center">
        <Text>{$eth.account.address}</Text>
    </div>
</section>