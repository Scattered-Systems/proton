<script>
	import Navbar from '$lib/nav/navbar/Navbar.svelte';
	import Toolbar from '$lib/nav/toolbar/Toolbar.svelte';
	import '$lib/styles.css';
	
	let bg = "bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900";
</script>

<Navbar/>
<div class="{bg} flex items-center text-white h-screen m-0 p-0">
	<div class="flex flex-col h-full m-0 p-0">
		<main class="flex flex-auto items-center justify-between h-full">
			<slot/>
		</main>
		<Toolbar/>
	</div>
</div>


