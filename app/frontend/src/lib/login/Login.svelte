<script>
    import { info } from '$lib/constants';
    import Text from '$lib/misc/Text.svelte';
    import { connected, defaultEvmStores } from 'svelte-web3';

    let props = {
        content: {
            ctrl: {
                submit: { label: "Submit" },
                wallet: { label: "Wallet" }
            }
        },
        styles: {
            buttons: {
                primary: "bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 px-3 py-1 rounded-full hover:opacity-75"
            }
        }
    }

    let formstore = {
        ens: "",
        password: ""
    }

    async function handle_submission() {
        
    }

	async function handle_auth() {
      if ($connected) {
        defaultEvmStores.disconnect();
      } else {
        defaultEvmStores.setProvider();
      }
      console.log($connected)
    }

    $: data = info;
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="A powerful, cloud-native application" />
</svelte:head>


<div class="flex flex-col justify-between m-3 p-3">
    <section class="p-3">
        <Text size={"xl"} props={"bold hover:opacity-75"}>Login</Text>
    </section>
    <div class="divide-y">
        <section class="items-center justify-between p-3">
            <form class="flex flex-col py-3">
                <input class="rounded-full py-1 px-3 text-black" id="inputUsername" value={formstore.ens}>
            </form>
            <button 
                class="{props.styles.buttons.primary}" 
                on:click={handle_submission}
                >
                {props.content.ctrl.submit.label}
            </button>
        </section>
        <section class="">
            <div class="flex grow items-center justify-center p-3">
                <button class="{props.styles.buttons.primary}" on:click={handle_auth}>
                    {props.content.ctrl.wallet.label}
                </button>
            </div>
        </section>
    </div>
</div>

<style>
	
</style>

