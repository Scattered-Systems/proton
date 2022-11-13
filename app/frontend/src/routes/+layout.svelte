<script>
	import '$lib/styles.css';
</script>

<svelte:head>
	<base href="/">
</svelte:head>

<div class="applicaiton bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 text-white">
	<div class="flex flex-col items-center justify-center min-h-screen z-0">
		<slot/>
	</div>
</div>

<style>
	.application {
		display: flex;
		flex-direction: column;
		margin: 0;
		min-height: 100vh;
		padding: 0;
	}
</style>
