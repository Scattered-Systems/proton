<script>
	import { connected } from 'svelte-web3';
	import Login from '$lib/login/Login.svelte';
	import Navbar from '$lib/nav/Navbar.svelte';
	import Toolbar from '$lib/toolbar/Toolbar.svelte';
	import '$lib/styles.css';
</script>

<div class="bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 text-white min-h-screen">
	{#if $connected }	
		<Navbar/>
		<div class="application z-0">
			<div class="flex flex-auto items-center justify-center mt-16 min-h-full">
				<main class="container mx-auto">
					<slot/>
				</main>
			</div>
			<Toolbar/>
		</div>
	{:else}
		<div class="application z-0">
			<div class="flex flex-auto items-center justify-center mt-16 min-h-full">
				<main class="container mx-auto">
					<Login/>
				</main>
			</div>
		</div>
	{/if}
</div>

<style>
	
</style>
