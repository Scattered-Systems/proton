<script>
    export let color = "text-black dark:text-white";
    export let size = "md";
    export let style = "normal";
</script>

<span class="flex min-w-full {color} text-{size} text-{style}"><slot/></span>