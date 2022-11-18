<script>
    export let props;

    export let align = {
        items: "items-center",
        justify: "justify-center"
    }
    export let bg = "";
    export let color = "";
    export let direction = "flex-col";
    export let display = "flex";
    export let m = "m-0";
    export let p = "p-0"
    export let sz = {
        h: "max-h-12",
        w: "w-24"
    };
    export let wrap = "flex-wrap";
    export let z = "";
</script>

<div class="{align.items} {align.justify} {bg} {color} {direction} {display} {m} {p} {sz.h} {sz.w} {z} {wrap}" class:props>
    <slot/>
</div>