<script>

    export let props;
    export let align = {
        items: "items-center",
        justify: "justify-center"
    };
    export let bg = "bg-transparent";
    export let color = "text-black dark:text-white";
    export let m = "mx-auto";
    export let p = "p-3";
    export let sz = {
        h: "h-full",
        w: "w-full"
    };

</script>

<div class="flex flex-col p-3 rounded mx-auto {align.items} {align.justify} {bg} {color} {m} {p} {sz.h} {sz.w}" class:props>
    <slot/>
</div> 



<style>
    
</style>