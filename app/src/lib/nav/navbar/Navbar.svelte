<script>
  import favicon from "$lib/assets/media/favicon.png";
  import { Navlink, NavLogo } from "$lib/nav/navbar";

  export let name = "";
  export const links = [];
</script>

<nav class="flex flex-nowrap items-center justify-center bg-transparent prose prose-invert min-w-full max-w-screen my-0 p-3 sticky top-0 z-50">
  <NavLogo logo={favicon}>
    {name}
  </NavLogo>
  <div class="lg:flex grow items-center justify-start sm:hidden xs:hidden" id="main-menu">
    <Navlink links={links}/>
  </div>
  <div class="justify-end">
    <slot/>
  </div>
</nav>

