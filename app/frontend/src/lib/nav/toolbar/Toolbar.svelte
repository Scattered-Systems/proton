<script>
    export let sidebar = false;
    import { page } from '$app/stores';
    import { info, colors, theme } from "$lib/constants.js";
    import List from '$lib/list/List.svelte';
    import Wallet from '$lib/wallet/Wallet.svelte';
    export let wrapper = 'absolute bg-zinc-800 bottom-0 flex items-center justify-between no-wrap opacity-95 w-screen z-20';

    let query = "";

    async function handle_submission() {
        
    }
</script>

<div class="{wrapper} toolbar p-3">
    <div class="divide-x flex no-wrap items-center justify-start mr-auto">
        <Wallet/>
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
            class="{colors.gradients.cyan[0]} rounded py-1 px-2.5 hover:opacity-95"
            on:click={handle_submission}
            >
            Search
        </button>
    </div>
</div>

<style>
    
</style>