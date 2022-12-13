<script>
    export let props;
    export let align = {
        items: "items-center",
        justify: "justify-center"
    }
    export let bg = "";
    export let color = "";
    export let dim = {
        h: "h-full",
        w: "w-full",
        z: ""
    };
    export let direction = "";
    export let m = "mx-auto";
    export let pd = "p-0"
    export let wrap = "flex-wrap";
</script>

<div class="flex {align.items} {align.justify} {bg} {color} {dim.h} {dim.w} {dim.z} {direction} {m} {pd} {wrap} {props}">
    <slot/>
</div>