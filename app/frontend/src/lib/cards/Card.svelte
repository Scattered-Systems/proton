<script>
    export let cls = "";

</script>

<div class="{cls}">
    <slot/>
</div>


<style>
    section {
        display: flex;
        flex-direction: column;
        margin: 0;
        min-height: 100%;
        padding: 0;
    }
</style>