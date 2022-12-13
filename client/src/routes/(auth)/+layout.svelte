<script>
    import { connected } from 'svelte-web3';
</script>

{#if $connected}
    <slot/>
{/if}