<script>
    import { page } from '$app/stores';
    import { info } from '$lib/constants';
    import { connected, defaultEvmStores } from 'svelte-web3';

    export let brand = "Portal";
    
    async function handle_auth() {
        defaultEvmStores.setProvider();
        console.log(connected)
    }
</script>

<nav class="bg-transparent text-white flex grow flex-nowrap items-center justify-between navbar navbar-expand-lg py-3 relative w-full">
    <div class="inline-flex w-32 px-3">
            { brand }
    </div>
    <div class="flex grow justify-start hover:text-underline hover:italic">
        <ul class="flex flex-col lg:flex-row list-none mr-auto">
            {#each info.pages as view}
                <li class="list-none" class:active={$page.url.pathname === view.href}>
                    <a class="block px-3 py-2 hover:opacity-75 dark:text-white" sveltekit:prefetch href="{view.endpoint}">
                        {view.label}
                    </a>
                </li>
              {/each}
          </ul>
    </div>
    <div class="mx-3">
        <button class="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 px-3 py-1 rounded-full hover:opacity-75" on:click={defaultEvmStores.setProvider}>
            {#if connected == true}
                {"Logout"}
            {:else}
                {"Connect"}
            {/if}
        </button>
    </div>
</nav>