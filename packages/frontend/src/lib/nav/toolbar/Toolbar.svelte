<script>
    export let align = {
        items: "items-center",
        justify: "justify-center"
    };
    export let bg = "bg-zinc-800 opacity-95";
    export let color = "text-white";
    export let position = "sticky bottom-0";
    export let sidebar = false;
    export let sz = {
        h: "",
        w: "min-w-full max-w-screen"
    }
    export let depth = "z-50";
    export let linktree = [];

    import SidebarToggle from '$lib/nav/sidebar/SidebarToggle.svelte';
    import SearchBtn from '$lib/buttons/search/SearchBtn.svelte';
    import ToolbarLinks from '$lib/nav/toolbar/ToolbarLinks.svelte';
    
    let search = "";

</script>

<div class="flex {align.items} {align.justify} {bg} {color} {depth} {position} {sz.h} {sz.w}">
    <div class="flex items-center justify-between no-wrap opacity-95 w-full">
        <div class="divide-x">
            <SidebarToggle bind:open={sidebar}/>
            <div class="px-3 items-center hidden lg:flex lg:grow">
                <ToolbarLinks links={linktree}/>
            </div>
        </div>
        <SearchBtn bg="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900" color={color} bind:query={search}/>
    </div>
</div>

<style>
    
</style>