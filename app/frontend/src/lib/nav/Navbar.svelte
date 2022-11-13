<script>
    import { page } from '$app/stores';
    import { info } from '$lib/constants';
    import { connected, defaultEvmStores } from 'svelte-web3';

    let props = {
      pages: [
        { href: "/", label: "Dashboard" }
      ]
    }
    let is_hidden = false;

    function setNavbarView() {
      is_hidden != is_hidden
    }
    
    async function handle_auth() {
      if ($connected) {
        defaultEvmStores.disconnect();
      } else {
        defaultEvmStores.setProvider();
      }
      console.log($connected)
    }

    $: data = info;
</script>

<nav class="absolute bg-transparent flex flex-nowrap items-center justify-between inset-x-0 mt-3 p-3 text-white top-0 w-full z-50">
  <div class="flex flex-initial w-1/6">
    <div class="">
      <a href="{data.homepage}" class="flex items-center">
        <img src="https://pzzld.eth.limo/media/img/Scattered-Systems-Logo.png" class="mr-3 h-6 sm:h-9 rounded-full" alt="#">
        <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">{data.name}</span>
      </a>
    </div>
  </div>
  <div class="lg:flex grow items-center justify-start sm:hidden xs:hidden" id="main-menu">
    <ul class="flex flex-col lg:flex-row list-none mr-auto">
      {#each props.pages as view}
          <li class:active={$page.url.pathname === view.endpoint}>
              <a class="block px-3 py-2 hover:opacity-75 dark:text-white" sveltekit:prefetch href="{view.endpoint}">
                  {view.label}
              </a>
          </li>
        {/each}
    </ul>
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

