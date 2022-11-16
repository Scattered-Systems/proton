<script>
    export let sidebar = false;

    import { page } from '$app/stores';
    import { info, theme } from "$lib/constants.js";
    import List from '$lib/list/List.svelte';
    import SidebarToggle from '$lib/nav/sidebar/SidebarToggle.svelte';

    let query = "";

    async function handle_submission() {
        
    }
</script>

<div class="absolute bg-zinc-800 bottom-0 flex items-center justify-between no-wrap opacity-95 w-screen z-50 p-3">
    <div class="divide-x flex no-wrap items-center justify-start mr-auto">
        <div class="px-3">
            <SidebarToggle bind:open={sidebar}/>
        </div>
        
        <div class="px-3 items-center hidden lg:flex lg:grow">
            <List props="flex flex-col lg:flex-row list-none">
                {#each info.sitemap.data[0].data as view}
                    <li class:active={$page.url.pathname === view.href}>
                        <a class="{theme.link}" sveltekit:prefetch href="{view.href}">
                            {view.label}
                        </a>
                    </li>
                {/each}
            </List>
        </div>
    </div>
    <div class="flex space-x-2">
        <input 
            class="flex items-center rounded-full mx-3 py-1 px-3 text-black hover:opacity-95"
            placeholder="Search"
            value={query}
            >
        <button 
            class="{theme.secondary} rounded py-1 px-2.5 hover:opacity-95"
            on:click={handle_submission}
            >
            Search
        </button>
    </div>
</div>

<style>
    
</style>