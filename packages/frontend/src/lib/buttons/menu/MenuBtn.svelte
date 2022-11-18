<script>
    export let align = {
        items: "items-center",
        justify: "justify-center"
    }
    export let bg = "";
    export let color = "";
    export let m = "mx-auto";
    export let p = "px-3 py-1"
    export let sz = {
        h: "max-h-12",
        w: "w-24"
    };
    export let open = false

    export function handle_click() {
        open = !open
    }
</script>

<button class="{align.items} {align.justify} {bg} {color} {m} {p} {sz.h} {sz.w} bottom-0 z-50" class:open on:click={handle_click}>
    <svg class="{align.items} {align.justify}">
        <line id="top" x1=0 y1=2 x2=32 y2=2/>
        <line id="middle" x1=0 y1=12 x2=32 y2=12/>
        <line id="bottom" x1=0 y1=22 x2=32 y2=22/>
    </svg>
</button>


<style>
    svg {
        display: flex;

        transition: transform 0.3s ease-in-out;
    }

    svg line {
        /* `currentColor` means inherit color from the text color */
        stroke: currentColor;
        stroke-width: 3;
        transition: transform 0.3s ease-in-out
    }
    /* rotate the top line */
    .open #top {
        transform: translate(6px, 0px) rotate(45deg)
    }

    /* hide the middle */
    .open #middle {
        opacity: 0;
    }

    /* rotate the bottom line */
    .open #bottom {
        transform: translate(-12px, 9px) rotate(-45deg)
    }
</style>