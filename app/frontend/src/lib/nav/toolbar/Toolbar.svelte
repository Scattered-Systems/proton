<script>
    import { page } from '$app/stores';
    import { info, colors } from "$lib/constants.js";
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
    
    export let button = {
        props: {
            color: colors.gradients.cyan[0],
            layout: "rounded-full px-3 py-1 rounded hover:opacity-75",
        },
        data: ["Search"]
    }
    let query = "";

    async function handle_submission() {
        
    }
    $: data = info;

</script>

<div class="{wrapper} toolbar p-3">
    <div class="divide-x flex no-wrap">
        <div class="no-wrap hidden sm:flex">
            <a class="{links.components.link}" href="/settings">
                <button>
                    Settings
                </button>
            </a>
        </div>
        <div class="items-center hidden lg:flex lg:grow">
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
            class="flex items-center rounded-full mx-3 py-1 px-3 text-black hover:opacity-75"
            placeholder="Search"
            value={query}
            >
        <button 
            class="{button.props.color} {button.props.layout}"
            on:click={handle_submission}
            >
            {button.data}
        </button>
    </div>
</div>

<style>

</style>