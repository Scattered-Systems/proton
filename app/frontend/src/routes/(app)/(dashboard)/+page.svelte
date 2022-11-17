<script>
    
    import HeaderBanner from '$lib/banners/header/HeaderBanner.svelte';

    /** @type {import('./$types').PageData} */
    export let data;
	let banner = "bg-cyan-600";
</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>


<div class="flex flex-auto items-center justify-between py-24 h-full w-full">
    
    <section class="bg-zinc-800 flex grow rounded h-full w-full">
        
        <div class="px-3 flex grow no-wrap items-center justify-center h-full w-full">
            <span class="prose">he4llo</span>
            
        </div>

    </section>
    <HeaderBanner bind:bg={banner}/>
</div>
