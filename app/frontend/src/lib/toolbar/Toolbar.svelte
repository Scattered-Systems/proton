<script>
    import { info } from "$lib/constants.js";

    let toolbar = {
        search: {
            label: "Search"
        }
    }

    let query = "";

    async function handle_submission() {
        
    }
    $: props = info;

</script>

<div class="bg-zinc-800 opacity-95 flex no-wrap items-center justify-between toolbar p-3 sticky bottom-0 z-0">
    <div class = "flex">
        <input 
            class="flex items-center rounded-full mx-3 py-1 px-3 text-black" 
            value={query}
            >
        <button 
            class="rounded-full bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900 px-3 py-1 rounded-full hover:opacity-75"
            >
            {toolbar.search.label}
        </button>
    </div>
    <div class = "xl:flex lg:flex md:hidden grow items-center sm:hidden xs:hidden">
        <ul class="flex flex-col lg:flex-row list-none mr-auto">
            <li><a class="hover:underline px-3 py-2 hover:opacity-75 dark:text-white" href="/">Activites</a></li>
            <li><a class="hover:underline px-3 py-2 hover:opacity-75 dark:text-white" href="/">Activites</a></li>
            <li><a class="hover:underline px-3 py-2 hover:opacity-75 dark:text-white" href="/">Activites</a></li>
            <li><a class="hover:underline px-3 py-2 hover:opacity-75 dark:text-white" href="/">Activites</a></li>
        </ul>
    </div>
    <div class = "flex no-wrap">
        <ul>
            <li><a class="hover:underline" href="/">Settings</a></li>
        </ul>
    </div>
</div>

<style>
    .toolbar {
		display: flex;
		margin: 0;
		min-width: 30vw;
	}
</style>