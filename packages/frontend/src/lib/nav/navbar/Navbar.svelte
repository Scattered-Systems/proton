<script>
  import { page } from '$app/stores';

  import NavLogo from './NavLogo.svelte';
  import { List } from '$lib/core';
  import { WalletToggle } from '$lib/core/misc';

  export let name = "";
  export const links = [];
  export let align = "items-center justify-center";
  export let bg = 'bg-transparent';
  export let color = 'text-white';
  export let dim = "min-w-full max-w-screen";
  export let m = "";
  export let pd = "p-3"
  export let position = "sticky top-0";
  export let space = "inset-x-0";
  export let z = 50;
</script>

<nav class="flex flex-nowrap {align} {bg} {color} {dim} {m} {pd} {position} {space} z-{z}">
  <div class="flex flex-auto items-center justify-center">
    <NavLogo>{name}</NavLogo>
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
      <WalletToggle/>
    </div>
  </div>
  
</nav>

