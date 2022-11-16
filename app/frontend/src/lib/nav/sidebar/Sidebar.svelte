<script>
    export let open = false;
    export let name = "Controls";

    
	import Text from '$lib/misc/text/Text.svelte';
	
</script>


<aside class="absolute bg-zinc-700 text-white flex max-w-md w-full items-center justify-between h-full opacity-95 top-0 z-40 py-24" class:open>
    <div class="flex flex-col justify-center h-full w-full">
        <div class="divide-y h-full">
            <div class="flex p-3 mx-auto items-center justify-center">
                <div class="flex p-3">
                    {name}
                </div>
                
            </div>
            <div class="flex flex-col justify-between py-3">
                <div class="flex flex-col h-full">
                    <slot/>
                </div>
            </div>
            <div class="flex flex-col justify-between py-3">
                <div class="flex flex-col grow">
                    <Text>
                        Settings
                    </Text>
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
