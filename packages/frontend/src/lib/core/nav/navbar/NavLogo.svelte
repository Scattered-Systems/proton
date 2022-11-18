<script>
    export let logo = {
        src: "https://pzzld.eth.limo/media/img/Scattered-Systems-Logo.png"
        };
</script>

<a class="flex max-w-sm px-3 py-1" href="/" sveltekit:prefetch>
    <button class="flex flex-auto nowrap hover:opacity-95 w-full">
        <img alt="#" class="mr-3 h-6 sm:h-9 rounded-full" src="{logo.src}" >
        <span class="self-center text-xl font-semibold whitespace-nowrap prose text-white">
            <slot/>
        </span>
    </button>
</a>


<style>

    
</style>