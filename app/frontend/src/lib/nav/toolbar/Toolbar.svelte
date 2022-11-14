<script>
    import { page } from '$app/stores';
    import { info } from "$lib/constants.js";
    import List from '$lib/list/List.svelte';
    let props = {
        search: {
            label: "Search"
        },
        links: {
            styles: {
                link: "hover:underline px-3 py-2 hover:opacity-75 dark:text-white",
                list: "flex flex-col lg:flex-row list-none mr-auto"
            },
            data: [
                { href: "/dashboard/account", label: "Account" },
                { href: "/dashboard/community", label: "Community" },
                { href: "/dashboard/content", label: "Content" },
                { href: "/dashboard/discover", label: "Discover" }
            ]
        }
    }

    let query = "";

    async function handle_submission() {
        
    }
    $: data = info;

</script>

<div class="bg-zinc-800 opacity-95 no-wrap items-center justify-between toolbar p-3 sticky bottom-0 z-0">
    <div class = "flex">
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
    <div class="items-center xs:hidden sm:hidden md:hidden lg:flex xl:flex lg:grow xl:grow">
        <List props="{props.links.styles.list}">
            {#each props.links.data as view}
                <li class:active={$page.url.pathname === view.href}>
                    <a class="{props.links.styles.link}" sveltekit:prefetch href="{view.href}">
                        {view.label}
                    </a>
                </li>
            {/each}
        </List>
    </div>
    <div class="flex no-wrap">
        <List>
            <li><a class="{props.links.styles.link}" href="/settings">Settings</a></li>
        </List>
    </div>
</div>

<style>
    .toolbar {
		display: flex;
		margin: 0;
		min-width: 100vw;
	}
</style>