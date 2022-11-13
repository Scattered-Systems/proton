<script>
  import List from '$lib/list/List.svelte';
  import NavBanner from '$lib/nav/navbar/banner/NavBanner.svelte';
  import { page } from '$app/stores';
  import { info } from '$lib/constants';
  import { connected, defaultEvmStores } from 'svelte-web3';
  let props = {
    banner: {
      href: "/",
      cls: "flex items-center"
    },
    links: []
  }
  
  async function handle_auth() {
      console.log($connected);

      if ($connected) {
          defaultEvmStores.disconnect();
          return Response.redirect('/', 301);
      } else {
          defaultEvmStores.setProvider();
          return Response.redirect('/dashboard', 301);
      }
  }

  $: data = info;
</script>

<nav class="absolute bg-transparent flex flex-nowrap items-center justify-between inset-x-0 mt-3 p-3 text-white top-0 w-full z-50">
  <NavBanner label="{{data: [data.name]}}"/>
  <div class="lg:flex grow items-center justify-start sm:hidden xs:hidden" id="main-menu">
    <List props="flex flex-col lg:flex-row list-none mr-auto">
      {#each props.links as view}
          <li class:active={$page.url.pathname === view.endpoint}>
              <a class="block px-3 py-2 hover:opacity-75 dark:text-white" sveltekit:prefetch href="{view.endpoint}">
                  {view.label}
              </a>
          </li>
        {/each}
    </List>
  </div>
  <div class="flex justify-end w-1/6">
    <button class="bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 px-3 py-1 rounded-full hover:opacity-75" on:click={handle_auth}>
        {#if $connected}
            {"Logout"}
        {:else}
            {"Login"}
        {/if}
    </button>
  </div>
</nav>

