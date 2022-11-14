<script>
    import Sidebar from "$lib/nav/sidebar/Sidebar.svelte";

	
</script>

<div class="flex no-wrap min-w-full">
	<Sidebar/>
	<section class="flex flex-auto grow items-center justify-between ml-auto p-3">
		<slot/>
	</section>
</div>



<style>
	
</style>
