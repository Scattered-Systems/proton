<script>
	import Navbar from '$lib/nav/Navbar.svelte';
	import '$lib/styles.css';
</script>

<div class="m-0 p-0 bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 min-h-screen text-white text-base">
	<Navbar/>
	<div class="flex flex-auto items-center justify-center">
		<main class="container mx-auto rounded bg-zinc-800 p-3 min-h-full">
			<slot />
		</main>
	</div>

	<footer class="flex flex-auto grow items-center justify-center relative w-full">
		<p>visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to learn SvelteKit</p>
	</footer>
</div>


<style>

	
</style>
