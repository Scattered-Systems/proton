<script>

    export let align = {
        items: "items-center",
        justify: "justify-center"
    }
    export let bg = "";
    export let color = "";
    export let direction = "";
    export let display = "container";
    export let m = "mx-auto";
    export let p = "p-0"
    export let sz = {
        h: "h-full",
        w: "w-full"
    };
    export let wrap = "flex-wrap";
    export let z = "";
</script>

<div class="{align.items} {align.justify} {bg} {color} {direction} {display} {m} {p} {sz.h} {sz.w} {wrap} {z}" >
    <slot/>
</div>