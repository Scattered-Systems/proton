<script>
    import { enhance } from "$app/forms";


    export let props = "m-0 py-3"
    export let method = "POST";
</script>

<form class="{props}" method={method} use:enhance><slot/></form>

<style>
    form {
        display: flex;
        flex-direction: column;
    }
</style>