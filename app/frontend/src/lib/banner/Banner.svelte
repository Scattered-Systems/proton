<script>
    import { theme } from '$lib/constants.js';
    export let label = "Banner";
    export let logo = {
        alt: "#",
        props: "mr-3 h-6 sm:h-9 rounded-full",
        src: "https://pzzld.eth.limo/media/img/Scattered-Systems-Logo.png"
    };
</script>

<div class="flex items-center justify-center">
    <a class="flex flex-initial" href="/" >
        <button class="{theme.button[0]}">
            <img src="{logo.src}" class="{logo.props}" alt="{logo.alt}">
            <span class="self-center text-xl font-semibold whitespace-nowrap">
                {label}
            </span>
        </button>
    </a>
</div>

<style>

    
</style>