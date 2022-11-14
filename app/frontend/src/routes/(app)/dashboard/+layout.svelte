<script>
	import Navbar from '$lib/nav/navbar/Navbar.svelte';
	import Toolbar from '$lib/nav/toolbar/Toolbar.svelte';
	import '$lib/styles.css';
</script>

<Navbar/>
<div class="flex flex-col items-center justify-center min-h-screen">
	<div class="flex flex-auto items-center justify-center min-h-full">
		<main class="container mt-16 mx-auto">
			<slot/>
		</main>
	</div>
	<Toolbar/>
</div>

<style>
	
</style>
