<script>
    export let open = false;
    export let bg = "bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 ";
    export let color = "text-white";
</script>

<!-- <div class="flex flex-initial items-center justify-center">
    <MenuBtn bind:open={open}/>
</div> -->

<button class="flex flex-initial rounded py-1 px-3 {bg} {color}" class:open on:click={() => open = !open}>
    Controls
</button>

<style>

</style>