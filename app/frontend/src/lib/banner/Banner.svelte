<script>
    export let label = "Banner";
    export let logo = {
        alt: "#",
        props: "mr-3 h-6 sm:h-9 rounded-full",
        src: "https://pzzld.eth.limo/media/img/Scattered-Systems-Logo.png"
    };
</script>

<div class="flex items-center justify-center">
    <a class="flex flex-initial" href="/" >
        <img src="{logo.src}" class="{logo.props}" alt="{logo.alt}">
        <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">
            {label}
        </span>
    </a>
</div>

<style>
</style>