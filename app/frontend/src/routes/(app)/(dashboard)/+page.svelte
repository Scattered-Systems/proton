<script>
    /** @type {import('./$types').PageData} */
    export let data;
</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>


<div class="flex flex-auto items-center justify-between py-24 h-full w-full">
    <section class="bg-zinc-800 flex grow rounded h-full w-full">
        <div class="px-3 flex grow no-wrap items-center justify-center h-full w-full">
            <span class="prose text-white">Welcome to Proton</span>
            
        </div>
    </section>
</div>
