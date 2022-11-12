<script>
    /** @type {import('./$types').PageData} */
    export let data;
    import Text from '$lib/misc/Text.svelte';
    import { connected, defaultEvmStores } from 'svelte-web3';

    let formstore = {
        ens: "",
        password: ""
    }

	async function handle_auth() {
      if ($connected) {
        defaultEvmStores.disconnect();
      } else {
        defaultEvmStores.setProvider();
      }
      console.log($connected)
    }

    $: props = data;
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="A powerful, cloud-native application" />
</svelte:head>


<div class="flex flex-col items-center justify-center p-3">
    <Text size={"xl"} props={"bold hover:opacity-75"}>Login</Text>
    <div class="flex flex-col items-center justify-between p-3">
        <form class="py-3">
            <input class="" value={formstore.ens}>
        </form>
        <button class="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 px-3 py-1 rounded-full hover:opacity-75" on:click={handle_auth}>
            Submit
        </button>
    </div>
    <div class="flex grow items-center justify-center p-3">
        <button class="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 px-3 py-1 rounded-full hover:opacity-75" on:click={handle_auth}>
            {"Wallet"}
        </button>
    </div>
</div>

<style>
	
</style>

