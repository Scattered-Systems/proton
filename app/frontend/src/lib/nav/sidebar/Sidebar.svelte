<script>
    /** @type {import('./$types').PageData} */
    export let data;
	import Text from '$lib/misc/text/Text.svelte';
    import { selectedAccount, web3 } from 'svelte-web3';

    const balance = async () => await $web3.eth.getBalance('0x0000000000000000000000000000000000000000');
	
</script>

<div class="bg-zinc-800 hidden xl:flex items-center justify-between rounded-lg">
    <div class="mx-auto p-3 sidebar">
        <div class="divide-y flex-col py-3">
            <Text>Sidebar</Text>
            <div class="flex flex-col grow mb-auto">
                
                <Text>
    
                    {$selectedAccount}
    
                </Text>
            </div>
            <div class="flex flex-col mt-auto">
                
                <Text>
                    Controls
    
                </Text>
            </div>
        </div>
        
    </div>
</div>

<style>
    .sidebar {
        height: 75vh
    }
</style>