<script>
    export let open = false;
    export let name = "Portal";

    
	import Text from '$lib/misc/text/Text.svelte';
	
</script>


<aside class="absolute bg-zinc-700 text-white hidden sm:flex max-w-md items-center justify-center h-full opacity-95 top-0 w-full z-40 pt-24" class:open>
    <div class="flex flex-col justify-center h-full w-full">
        <div class="divide-y h-full">
            <div class="flex p-3 mx-auto items-center justify-center">
                <div class="flex p-3">
                    Control Panel
                </div>
                
            </div>
            <div class="flex flex-col justify-between py-3">
                <div class="flex flex-col grow mb-auto">
                    <slot/>
                </div>
                <div class="flex flex-col mt-auto">
                    <Text>Controls</Text>
                </div>
            </div>
        </div>
        
    </div>
</aside>

<style>
    aside {
        left: -100%;
        transition: left 0.3s ease-in-out
    }

    .open {
        left: 0
    }
</style>
