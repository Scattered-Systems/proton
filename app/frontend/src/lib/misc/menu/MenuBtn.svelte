<script>
    import { theme } from '$lib/constants.js';
    export let open = false

    function handle_click() {
        open = !open
    }
</script>

<div class="flex flex-initial items-center jsutify-center">
    <button class="flex space-x-2" class:open on:click={handle_click}>
        <svg class="inline-flex">
            <line id="top" x1=0 y1=2 x2=32 y2=2/>
            <line id="middle" x1=0 y1=12 x2=32 y2=12/>
            <line id="bottom" x1=0 y1=22 x2=32 y2=22/>
        </svg>
    </button>
</div>


<style>
    svg {
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