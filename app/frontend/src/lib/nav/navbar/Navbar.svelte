<script>
  import { page } from '$app/stores';
  import { info } from '$lib/constants';

  import List from '$lib/list/List.svelte';
  import Banner from '$lib/banner/Banner.svelte';
  import Wallet from '$lib/login/wallet/Wallet.svelte';

  export const links = [];

</script>

<nav class="absolute bg-transparent flex flex-nowrap items-center justify-between inset-x-0 mt-3 p-3 text-white top-0 w-full z-50">
  <div class="flex xl:hidden">
    <Banner label="{info.name}"/>
  </div>
  
  <div class="lg:flex grow items-center justify-start sm:hidden xs:hidden" id="main-menu">
    <List props="flex flex-col lg:flex-row list-none mr-auto">
      {#each links as view}
          <li class:active={$page.url.pathname === view.endpoint}>
              <a class="block px-3 py-2 hover:opacity-75 dark:text-white" sveltekit:prefetch href="{view.endpoint}">
                  {view.label}
              </a>
          </li>
        {/each}
    </List>
  </div>
  <div class="justify-end">
    <Wallet/>
  </div>
</nav>

