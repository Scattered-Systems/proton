<script>
    import Card from '$lib/cards/card/Card.svelte';
    import CardBody from '$lib/cards/card/CardBody.svelte';
    import Section from '$lib/sections/Section.svelte';
    /** @type {import('./$types').PageData} */
    export let data;
    let card = {
        bg: "bg-zinc-500",
        color: 'text-white'
    }

    import { connected, defaultEvmStores, selectedAccount } from 'svelte-web3';
    $: web3 = defaultEvmStores.web3;

</script>

<svelte:head>
	<title>{data.head.title}</title>
	<meta name="description" content="{data.head.description}" />
</svelte:head>


<div class="flex flex-auto items-center justify-between py-24 h-full w-full">
    <Section bg="bg-transparent" dir="flex-row">
        <Card bg={card.bg} color={card.color}>
            <CardBody>
                <span class="prose text-center">
                    Welcome to Proton
                </span>
            </CardBody>
        </Card>
        <Card bg={card.bg} color={card.color}>
            <CardBody>
                <span class="prose text-center">
                    Welcome to Proton
                </span>
            </CardBody>
        </Card>
        
    </Section>
</div>
