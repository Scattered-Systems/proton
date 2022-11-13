<script>
    import Form from '$lib/form/Form.svelte';
    import Text from '$lib/misc/text/Text.svelte';
    import { connected, defaultEvmStores } from 'svelte-web3';

    let props = {
        content: {
            ctrl: {
                submit: { label: "Submit" },
                wallet: { label: "Wallet" }
            }
        },
        forms: {
            stores: [
                {
                    cls: "rounded-full py-1 px-3 text-black",
                    id: "inputUsername",
                    label: "username",
                    value: ""
                },
                {
                    id: 1,
                    label: "password",
                    value: ""
                }
            ],
            styles: {
                input: {
                    bd: "rounded-full",
                    color: "text-black",
                    pd: "py-1 px-3",
                }
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

    let styler = () => {
        {
            res: ""
        }
    }

    async function handle_submission() {
        
    }

	async function handle_auth() {
      if ($connected) {
        defaultEvmStores.disconnect();
        return Response.redirect('/', 200);
      } else {
        defaultEvmStores.setProvider();
        return Response.redirect('/dashboard', 200);
      }
      console.log($connected)
    }
</script>

<div class="flex flex-col justify-between m-3 p-3 min-h-full">
    <section class="p-3">
        <Text size={"xl"} props={"bold hover:opacity-75"}>Login</Text>
    </section>
    <div class="divide-y">
        <section class="items-center justify-between p-3">
            <Form props="py-3">
                <input class="{props.forms.stores[0].cls}" id="{props.forms.stores[0].id}" value={props.forms.stores[0].value}>
            </Form>
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

