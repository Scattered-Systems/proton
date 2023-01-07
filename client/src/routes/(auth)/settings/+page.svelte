<script>
    let query = "";

    function handle_click() {

    }
</script>

<div class="flex grow h-full w-full">
    <form class="flex flex-col min-h-full min-w-full ">
        <select class="px-4 py-3 rounded-full">
            A
        </select>
        <input 
            class="flex items-center rounded-full mx-3 py-1 px-3 text-black hover:opacity-95"
            placeholder="Search"
            value={query}
            >
        <button 
            class="rounded py-1 px-2.5 hover:opacity-95"
            on:click={handle_click}
            >
            Search
        </button>
    </form>
</div>