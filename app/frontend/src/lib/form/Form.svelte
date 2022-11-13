<script>
    export let props = "m-0 py-3"

</script>

<form class="{props}"><slot/></form>

<style>
    form {
        display: flex;
        flex-direction: column;
    }
</style>