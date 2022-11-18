<script>
	import { info } from '$lib/constants.js';
	import { Navbar, Sidebar, Toolbar } from '$lib/nav';
	import '$lib/styles.css';
	
	let bg = "bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900";
	let open = false;

</script>

<Navbar bind:sidebar={open} name={info.name}/>
<div class="{bg} flex items-center text-white min-h-screen m-0 p-0 z-0">
	<div class="flex flex-col h-full w-full m-0 p-0">
		<main class="flex flex-col items-center justify-center h-full w-full z-10">
			<slot/>
		</main>
		
	</div>
</div>
<Toolbar linktree={info.sitemap.data[0].data} bind:sidebar={open}/>
<Sidebar bind:open>
	Sidebar
</Sidebar>

