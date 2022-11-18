<script>
    export let align = {
        items: "items-center",
        justify: "justify-center"
    };
    export let bg = "bg-transparent";
    export let color = "text-black dark:text-white";
    export let dir = "flex-col";
    export let m = "m-0";
    export let p = "p-0";
    export let sz = {
        h: "h-full",
        w: "w-full"
    };
</script>

<section class="flex {dir} {align.items} {align.justify} {bg} {color} {m} {p} {sz.h} {sz.w}"><slot/></section>


<style>
</style>