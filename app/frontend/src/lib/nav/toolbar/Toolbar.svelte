<script>
    import { page } from '$app/stores';
    import { info } from "$lib/constants.js";
    import List from '$lib/list/List.svelte';
    export let wrapper = 'bg-zinc-800 bottom-0 flex items-center justify-between no-wrap opacity-95 sticky w-screen z-50';
    export let links = {
        components: {
            elem: "",
            link: "hover:underline px-3 py-2 hover:opacity-75 text-white",
            list: "flex flex-col lg:flex-row list-none mr-auto"
        },
        data: info.sitemap.data[0].data
    }
    let props = {
        search: {
            label: "Search"
        },
        links: {
            styles: {
                link: "hover:underline px-3 py-2 hover:opacity-75 dark:text-white",
                list: "flex flex-col lg:flex-row list-none mr-auto"
            },
            data: info.sitemap.data[0].data
        }
    }

    let query = "";

    async function handle_submission() {
        
    }
    $: data = info;

</script>

<div class="{wrapper} toolbar p-3">
    <div class="divide-x flex no-wrap">
        <div class="flex no-wrap">
            <a class="{links.components.link}" href="/settings">
                <button>
                    Settings
                </button>
            </a>
        </div>
        <div class="items-center xs:hidden sm:hidden md:hidden lg:flex xl:flex grow">
            <List props="{links.components.list}">
                {#each links.data as view}
                    <li class:active={$page.url.pathname === view.href}>
                        <a class="{links.components.link}" sveltekit:prefetch href="{view.href}">
                            {view.label}
                        </a>
                    </li>
                {/each}
            </List>
        </div>
    </div>
    <div class="flex">
        <input 
            class="flex items-center rounded-full mx-3 py-1 px-3 text-black"
            placeholder="Search"
            value={query}
            >
        <button 
            class="rounded-full bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 px-3 py-1 rounded-full hover:opacity-75"
            on:click={handle_submission}
            >
            {props.search.label}
        </button>
    </div>
</div>

<style>

</style>