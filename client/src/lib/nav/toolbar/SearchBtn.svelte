<script>
    export let bg = "bg-gradient-to-r from-cyan-700 via-cyan-500 to-cyan-900";
    export let color = "text-white";
    export let query = "";
    
    function handle_click() {

    }
</script>

<input 
    class="flex items-center rounded-full mx-3 py-1 px-3 text-black hover:opacity-95"
    placeholder="Search"
    value={query}
    >
<button 
    class="{bg} {color} rounded py-1 px-2.5 hover:opacity-95"
    on:click={handle_click}
    >
    Search
</button>