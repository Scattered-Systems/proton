<script>
    import init, { greet } from 'proton';
    import { onMount } from 'svelte';

    onMount(() => {
        console.log('init wasm-pack');
        greet('From svelte-kit');
    })
    
</script>

<svelte:head>
	<title>Dashboard</title>
	<meta name="description" content="Welcome to puzzled, a proof-of-concept application written in support of the Scattered-Systems, DAO LLC ecosystem" />
</svelte:head>


<div class="flex flex-auto items-center justify-center rounded">
    <span class="prose text-center text-white">
        <h1 class="text-2xl prose text-white">Puzzled</h1>
        
    </span>
</div>