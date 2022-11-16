<script>
    /** @type {import('./$types').PageData} */
    export let data;
	
</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>

<section class="flex items-center justify-center h-full w-full">
    <div class="flex grow mx-auto p-3 ld:w-5/6">
        <div class="bg-zinc-800 rounded p-3 flex grow no-wrap items-center justify-center min-h-full">
            <span class="">Content Section</span>
        </div>
    </div>
    
</section>
