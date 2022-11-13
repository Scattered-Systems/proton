<script>
	import Navbar from '$lib/nav/navbar/Navbar.svelte';
	import Toolbar from '$lib/nav/toolbar/Toolbar.svelte';
	import '$lib/styles.css';
</script>

<Navbar/>
<div class="flex flex-col min-h-screen">
	<div class="flex flex-auto items-center justify-center mt-16 min-h-full">
		<main class="container mx-auto">
			<slot/>
		</main>
	</div>
	<Toolbar/>
</div>

<style>
	
</style>
