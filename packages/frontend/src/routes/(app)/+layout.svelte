<script>
	import { info } from '$lib/constants.js';
	import { Navbar, Sidebar, Toolbar } from '$lib/core/nav';
	import '$lib/styles.css';
	
	let bg = "bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900";
	let open = false;

</script>


<div class="{bg} flex flex-col items-center text-white min-h-screen m-0 p-0 z-0">
	<Navbar bind:sidebar={open} name={info.name}/>
	<main class="flex flex-col grow items-center justify-center min-h-full max-h-screen min-w-full max-w-full z-10">
		<slot/>
	</main>
	<Toolbar linktree={info.sitemap.data[0].data} bind:sidebar={open}/>
</div>

<Sidebar bind:open>
	Sidebar
</Sidebar>
