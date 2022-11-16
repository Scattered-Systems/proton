<script>
    /** @type {import('./$types').PageData} */
    export let data;
	
</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>

<section class="flex nowrap items-center justify-center py-24 h-full w-full">
    <div class="flex bg-zinc-800 grow mx-auto rounded h-full w-full ">
        <div class=" p-3 flex grow no-wrap items-center justify-center h-full w-full">
            <span class="">Content Section</span>
        </div>
    </div>
    
</section>
