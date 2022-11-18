<script>
    export let links = [];
    
    import { page } from '$app/stores';
    import { List } from '$lib/core';
</script>

<List props="flex flex-col lg:flex-row list-none">
    {#each links as view}
        <li class:active={$page.url.pathname === view.href}>
            <a class="flex flex-initial overflow-y-clip hover:underline px-3 py-2 hover:opacity-75 text-white" sveltekit:prefetch href="{view.href}">
                {view.label}
            </a>
        </li>
    {/each}
</List>