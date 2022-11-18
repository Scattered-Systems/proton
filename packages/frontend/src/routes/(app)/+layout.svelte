<script>
	import { info } from '$lib/constants.js';
	import Navbar from '$lib/nav/navbar/Navbar.svelte';
	import Sidebar from "$lib/nav/sidebar/Sidebar.svelte";
	import Toolbar from '$lib/nav/toolbar/Toolbar.svelte';
	import '$lib/styles.css';
	
	let bg = "bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900";
	let open = false;
</script>

<Navbar bind:sidebar={open} name={info.name}/>
<div class="{bg} flex items-center text-white h-screen m-0 p-0 z-0">
	<div class="flex flex-col h-full w-full m-0 p-0">
		<main class="flex items-center justify-center h-full w-full">
			<slot/>
		</main>
		<Toolbar bind:sidebar={open}/>
	</div>
</div>


<Sidebar bind:open>
	Sidebar
</Sidebar>

