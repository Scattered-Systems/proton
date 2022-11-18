<script>
    /** @type {import('./$types').PageData} */
    export let data;
    /** @type {import('./$types').ActionData} */
	export let form;
    import { info, colors } from '$lib/constants';
    import { Form, Text } from '$lib/core';
    import { WalletToggle } from '$lib/core/misc';


    let btn_primary = info.theme.button.layout.primary;
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
                    <WalletToggle/>
                </div>
            </Form>
            
        </section>
        <section class="">
            <div class="items-center justify-between p-3">
                <a href="/register">
                    <button class="{colors.gradients.primary} {btn_primary}">
                        Create a new account
                    </button>
                </a>
            </div>
        </section>
    </section>
</div>

<style>
	
</style>