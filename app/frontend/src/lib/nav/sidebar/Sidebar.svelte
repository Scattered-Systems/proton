<script>
    /** @type {import('./$types').PageData} */
    export let data;
	import Text from '$lib/misc/text/Text.svelte';
    import { selectedAccount, web3 } from 'svelte-web3';

    const balance = async () => await $web3.eth.getBalance('0x0000000000000000000000000000000000000000');
	
</script>


<div class="bg-zinc-800 hidden xl:flex items-center justify-between m-0 h-full py-3">
    <div class="flex flex-col items-center justify-between h-full py-12">
        <div class="divide-y">
            <div class="flex flex-col flex-initial">
                <span class="h2 text-center">Sidebar</span>
            </div>
            <div class="flex-col py-3">
                
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
    
</div>
