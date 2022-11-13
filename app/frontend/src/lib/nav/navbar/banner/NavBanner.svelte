<script>
    export let label = { data: ["Banner"]};
    export let logo = {
        alt: "#",
        src: "https://pzzld.eth.limo/media/img/Scattered-Systems-Logo.png"
    };
</script>

<div class="banner flex-initial w-1/6">
    <div class="">
        <a class="flex items-center justify-center" href="/dashboard" >
            <img src="{logo.src}" class="mr-3 h-6 sm:h-9 rounded-full" alt="{logo.alt}">
            <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">
                {label.data}
            </span>
        </a>
    </div>
</div>

<style>
    .banner {
        display: flex;
    }
</style>