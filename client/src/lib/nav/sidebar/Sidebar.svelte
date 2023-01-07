<script>
    export let open = false;
    export let title = "Controls";

    import { ProfileCard } from "$lib";
	
</script>


<aside class="absolute bg-zinc-700 text-white flex max-w-md w-full items-center justify-between h-full opacity-95 top-0 z-40 py-24" class:open>
    <div class="flex flex-col justify-between h-full w-full">
        <div class="divide-y h-full">
            <div class="flex p-3 mx-auto items-center justify-center">
                {title}
            </div>
            <div class="flex p-3 mx-auto items-center justify-start">
                <ul class="flex flex-col lg:flex-row list-none">
                    <li>
                        <a class="" href="/settings" sveltekit:prefetch>
                            Settings
                        </a>
                    </li>
                </ul>
            </div>
            <div class="flex p-3">
                <ProfileCard/>
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
