<script>
    /** @type {import('./$types').PageData} */
    export let data;
    /** @type {import('./$types').ActionData} */
	export let form;
    import { info } from '$lib/constants';
    import Form from '$lib/form/Form.svelte';
    import Text from '$lib/misc/text/Text.svelte';
    import Wallet from '$lib/login/wallet/Wallet.svelte';

    let btn_primary = [
        info.theme.color.gradient.primary,
        info.theme.button.layout.primary,
    ]
</script>

<svelte:head>
    <title>Proton - {data.title}</title>
</svelte:head>

<div class="divide-y flex flex-col justify-between m-3 p-3 min-h-full">
    <section class="p-3">
        <Text size={"xl"} props={"bold hover:opacity-75"}>
            Login
        </Text>
    </section>
    <section>
        <section class="items-center justify-between p-3">
            <Form props="py-3">
                
                <div class="items-center justify-between p-3">
                    <Wallet/>
                </div>
            </Form>
            
        </section>
        <section class="">
            <div class="items-center justify-between p-3">
                <a href="/register">
                    <button class="{btn_primary[0]} {btn_primary[1]}">
                        Create a new account
                    </button>
                </a>
            </div>
        </section>
    </section>
</div>

<style>
	
</style>