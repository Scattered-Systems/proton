<script>
	import Footer from '$lib/footer/Footer.svelte';
	import Navbar from '$lib/nav/Navbar.svelte';
	import '$lib/styles.css';
</script>

<Navbar/>
<div class="application bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 text-white">
	<div class="flex flex-auto items-center justify-center mt-16 min-h-full">
		<main class="bg-zinc-800 container mx-auto min-h-full">
			<slot />
		</main>
	</div>
</div>
<Footer/>


<style>
	
</style>
