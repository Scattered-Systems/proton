<script>
    /** @type {import('./$types').PageData} */
    export let data;
    /** @type {import('./$types').ActionData} */
	export let form;

    import { info } from '$lib/constants';
    import Form from '$lib/form/Form.svelte';
    import Text from '$lib/misc/text/Text.svelte';

    let btn_primary = [
        info.theme.color.gradient.primary,
        info.theme.button.layout.primary,
    ]
    
    $: props = data;
</script>

<svelte:head>
    <title>Proton - {props.title}</title>
</svelte:head>

<div class="divide-y flex flex-col justify-between m-3 p-3 min-h-full">
    <section class="p-3">
        <Text size={"xl"} props={"bold hover:opacity-75"}>
            {props.title}
        </Text>
    </section>
    <div class="">
        <section class="items-center justify-between p-3">
            <Form>
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
            <div class="items-center justify-between p-3">
                <a href="/login">
                    <button class="{btn_primary[0]} {btn_primary[1]}">
                        Have an account already?
                    </button>
                </a>
            </div>
        </section>
    </div>
</div>