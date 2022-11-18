<script>
    import { Flex } from '$lib/core';
    export let props;
    export let align = {
        items: "items-center",
        justify: "justify-center"
    };
    export let bg = "bg-transparent";
    export let color = "text-black dark:text-white";
    export let m = "mx-auto";
    export let pd = "p-3";
    export let dim = {
        h: "h-full",
        w: "w-full",
        z: ""
    };

</script>

<div class="rounded-lg {dim.h} {dim.w} {m} {pd} ">
    <Flex align={align} bg={bg} color={color} dim={dim} direction="flex-col">
        <slot/>
    </Flex> 
</div>




<style>
    
</style>