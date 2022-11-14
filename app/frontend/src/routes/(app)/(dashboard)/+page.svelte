<script>
    /** @type {import('./$types').PageData} */
    export let data;
    import { chainId, connected, selectedAccount } from 'svelte-web3';
	
</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>

<section class="flex flex-auto no-wrap items-center justify-center min-h-full">
    Content Section
</section>