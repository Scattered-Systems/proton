<script>
    /** @type {import('./$types').PageData} */
    export let data;
	
</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>

<section class="flex flex-auto no-wrap items-center justify-between min-h-full mx-auto">
    <div class="flex flex-col grow no-wrap items-center justify-center">
        <span>Content Section</span>
    </div>
    <div class="md:flex flex-col wrap items-center justify-center hidden">
        <span>Feed</span>
    </div>
    
</section>