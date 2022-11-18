<script>
    import MenuBtn from '$lib/buttons/menu/MenuBtn.svelte';
    export let open = false;
    export let theme = "bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 text-white";

</script>

<div class="flex flex-initial items-center justify-center">
    <MenuBtn bind:open={open}/>
</div>
<!-- <button class="flex rounded py-1 px-3 {theme}" class:open on:click={() => open = !open}>
    Control Panel
</button> -->

<style>

</style>