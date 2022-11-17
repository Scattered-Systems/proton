<script>
    import ProfileCard from '$lib/cards/profile/ProfileCard.svelte';

    /** @type {import('./$types').PageData} */
    export let data;
	
</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>

<div class="flex nowrap items-center justify-between py-24 h-full w-full">
    <ProfileCard/>
    <div class="flex grow mx-auto h-full w-full px-3">
        <section class="bg-zinc-800 rounded h-full w-full">
            <div class=" p-3 flex grow no-wrap items-center justify-center h-full w-full">
            
            
            </div>

        </section>
    </div>
    
</div>
