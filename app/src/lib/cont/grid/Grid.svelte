<script>
    export let align = {
        items: "items-center",
        justify: "justify-center"
    }
    export let bg = "";
    export let color = "";
    export let depth = ""
    export let direction = "flex-row";
    export let m = "m-0";
    export let p = "p-0"
    export let sz = {
        h: "max-h-12",
        w: "w-24"
    };
    export let wrap = "flex-wrap";
    export let z = "";
</script>

<div class="flex {align.items} {align.justify} {bg} {color} {depth} {direction} {m} {p} {sz.h} {sz.w} {wrap} {z}">
    <slot/>
</div>