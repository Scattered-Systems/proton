<script>

    /** @type {import('./$types').PageData} */
    export let data;
    /** @type {import('./$types').ActionData} */
	export let form;

    import { info } from '$lib/constants';
    import Form from '$lib/form/Form.svelte';
    import Text from '$lib/misc/text/Text.svelte';
    import Wallet from '$lib/login/wallet/Wallet.svelte';
    
</script>

<svelte:head>
    <title>Proton - Login</title>
</svelte:head>

<div class="flex flex-col justify-between m-3 p-3 min-h-full">
    <section class="p-3">
        <Text size={"xl"} props={"bold hover:opacity-75"}>
            Login
        </Text>
    </section>
    <div class="divide-y">
        <section class="items-center justify-between p-3">
            <Form props="py-3">
                <fieldset class="form-group py-3">
                    <input 
                        class="{info.theme.form.layout[0]}" 
                        id="{data.form[0].data[0].id}"
                        name="username"
                        type="string"
                        placeholder="Username"
                    />
                </fieldset>
                
                <fieldset class="form-group py-3">
                    <input
                        class="{info.theme.form.layout[0]}"
                        id="formInputPassword"
                        name="password"
                        type="password"
                        required
                        placeholder="Password"
                    />
                </fieldset>
                <button 
                    class="{info.theme.color.gradient.primary} {info.theme.button.layout.primary}"
                    type="submit"
                >
                    {data.ctrl.submit.label}
                </button>
            </Form>
            
        </section>
        <section class="">
            <div class="flex items-center justify-between p-3">
                <Wallet/>
            </div>
        </section>
    </div>
</div>

<style>
	
</style>