<script>
    export let align = {
        items: "items-center",
        justify: "justify-center"
    };
    export let bg = "bg-transparent";
    export let color = "text-black dark:text-white";
    export let dim = {
        h: "min-h-full max-h-screen",
        w: "min-w-full max-w-screen",
        z: ""
    };
    export let dir = "flex-col";
    export let m = "m-0";
    export let pd = "p-0";
    export let props = ""
</script>

<section class="flex {dir} {align.items} {align.justify} {bg} {color} {m} {pd} {dim.h} {dim.w} {dim.z} {props}"><slot/></section>

<style>
</style>