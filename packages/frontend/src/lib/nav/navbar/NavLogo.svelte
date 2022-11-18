<script>
    export let logo = {
        src: "https://pzzld.eth.limo/media/img/Scattered-Systems-Logo.png"
        };
</script>

<button class="flex max-w-sm px-3 py-1">
    <a class="inline-block nowrap hover:opacity-95 w-full" href="/" sveltekit:prefetch>
        <img alt="#" class="mr-3 h-6 sm:h-9 rounded-full" src="{logo.src}" >
        <span class="self-center text-xl font-semibold whitespace-nowrap prose">
            <slot/>
        </span>
    </a>
</button>


<style>

    
</style>