<script>
    export let color = "text-black dark:text-white";
    export let size = "md";
    export let props = "normal";
</script>

<span class="flex min-w-full {color} text-{size} {props}"><slot/></span>