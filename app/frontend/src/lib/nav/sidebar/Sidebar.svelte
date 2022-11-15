<script>
    /** @type {import('./$types').PageData} */
    export let data;

    import { info } from '$lib/constants.js';
    import Banner from '$lib/banner/Banner.svelte';
	import Text from '$lib/misc/text/Text.svelte';
    import { selectedAccount, web3 } from 'svelte-web3';

    const balance = async () => await $web3.eth.getBalance('0x0000000000000000000000000000000000000000');
	
</script>


<div class="bg-zinc-800 hidden xl:flex items-center justify-center h-full w-full">
    <div class="divide-y flex flex-col h-full">
        <div class="hidden xl:flex p-3 mx-auto items-center justify-center">
            <div class="flex p-3">
                <Banner label="{info.name}"/>
            </div>
            
        </div>
        <div class="flex flex-col justify-between py-3">
            <div class="flex flex-col grow mb-auto">
                <Text>
                    {$selectedAccount}
                </Text>
            </div>
            <div class="flex flex-col mt-auto">
                <Text>Controls</Text>
            </div>
        </div>
    </div>
</div>
