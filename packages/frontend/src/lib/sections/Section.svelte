<script>
    export let cls = "";

</script>

<section class="{cls}"><slot/></section>


<style>
    section {
        display: flex;
        flex-direction: column;
        margin: 0;
        min-height: 100%;
        padding: 0;
    }
</style>