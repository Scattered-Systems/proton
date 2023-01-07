<script>
    import { fetch } from 'wasm/proton';
    import { onMount } from 'svelte';

    onMount(() => {
        console.log(fetch("https://google.com"));
        console.log('init wasm-pack');
        greet('From svelte-kit');
    })
    
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="Welcome to the Proton early access program" />
</svelte:head>


<div class="flex flex-auto items-center justify-center rounded">
    <span class="prose text-center text-white">
        <h1 class="text-2xl prose text-white">Proton</h1>
    </span>
</div>