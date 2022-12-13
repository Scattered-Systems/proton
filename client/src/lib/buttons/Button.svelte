<script>
export let align = {
        items: "items-center",
        justify: "justify-center"
    }
    export let bg = "";
    export let color = "";
    export let depth = "z-50"
    export let m = "mx-auto";
    export let p = "px-3 py-1"
    export let sz = {
        h: "max-h-12",
        w: "w-24"
    };
    export let open = false

    function handle_click() {
        open = !open
    }
</script>

<button class="{align.items} {align.justify} {bg} {color} {depth} {m} {p} {sz.h} {sz.w}">
    <slot/>
</button>