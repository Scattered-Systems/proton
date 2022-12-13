<script>
    import { SidebarToggle } from '$lib';
    import SearchBtn from './SearchBtn.svelte';
    import ToolbarLinks from './ToolbarLinks.svelte';

    export let linktree = [];
    let search = "";
    export let sidebar = false;

    export let align = {
        items: "items-center",
        justify: "justify-between"
    };
    export let bg = "bg-transparent";
    export let color = "text-white";
    export let m = "m-0";
    export let pd = "p-3";
    export let position = "sticky bottom-0";
    export let dim = {
        h: "",
        w: "min-w-full max-w-screen",
        z: "z-50"
    }
</script>

<div class="flex flex-nowrap {align.items} {align.justify} {bg} {color} {dim.h} {dim.w} {dim.z} {m} {pd} {position}">
    <div class="divide-x flex flex-nowrap grow">
        <div class="px-3 items-center hidden sm:flex">
            <SidebarToggle bind:open={sidebar}/>
        </div>
        
        <div class="px-3 items-center hidden lg:flex">
            <ToolbarLinks links={linktree}/>
        </div>
    </div>
    <SearchBtn bg="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900" color={color} bind:query={search}/>
</div>

<style>
    
</style>