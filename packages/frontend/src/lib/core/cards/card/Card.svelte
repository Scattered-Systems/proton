<script>
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

<div class="flex flex-col rounded-lg {align.items} {align.justify} {bg} {color} {dim.h} {dim.w} {dim.z} {m} {pd} {props}">
    <div class="flex min-h-full max-h-screen min-w-full max-w-screen p-3">
        <slot/>
    </div> 
</div>




<style>
    
</style>