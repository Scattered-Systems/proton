<script>
    /** @type {import('./$types').PageData} */
    export let data;
	
</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>

<section class="flex flex-auto no-wrap items-center justify-center min-h-full">
    <div class="bg-zinc-800 rounded p-3 min-h-full">
        <div class="flex grow no-wrap items-center justify-center min-h-full">
            <span class="">Content Section</span>
        </div>
    </div>
    <div class="md:flex flex-col wrap items-center justify-center hidden ml-auto">
        <span>Feed</span>
    </div>
    
</section>