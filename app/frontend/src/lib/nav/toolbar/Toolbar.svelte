<script>
    import { page } from '$app/stores';
    import { info, colors, theme } from "$lib/constants.js";
    import List from '$lib/list/List.svelte';
    export let wrapper = 'bg-zinc-800 bottom-0 flex items-center justify-between no-wrap opacity-95 sticky w-screen';

    let query = "";

    async function handle_submission() {
        
    }
</script>

<div class="{wrapper} toolbar p-3">
    <div class="divide-x flex no-wrap">
        <div class="hidden md:flex">
            <a class="{theme.link}" href="/settings">
                <button class="{theme.button.primary}">
                    <svg class="inline-block h-sm">
                        <line id="top" x1=0 y1=2 x2=32 y2=2/>
                        <line id="middle" x1=0 y1=12 x2=32 y2=12/>
                        <line id="bottom" x1=0 y1=22 x2=32 y2=22/>
                    </svg>
                </button>
            </a>
        </div>
        <div class="items-center hidden lg:flex lg:grow">
            <List props="flex flex-col lg:flex-row list-none mr-auto">
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
    svg {
        transition: transform 0.3s ease-in-out;
    }

    svg line {
        /* `currentColor` means inherit color from the text color */
        stroke: #000;
        stroke-width: 3;
        transition: transform 0.3s ease-in-out;
    }

    .open svg {
        transform: scale(0.7);
    }
</style>