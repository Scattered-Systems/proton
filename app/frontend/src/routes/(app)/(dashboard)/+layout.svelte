<script>
    import Sidebar from "$lib/nav/sidebar/Sidebar.svelte";

	
</script>

<div class="grid grid-flow-row-dense grid-cols-4">
	<div class="col-span-1">
		<Sidebar/>
	</div>
	<div class="col-span-3">
		<section class="flex flex-auto grow items-center justify-center p-3">
			<slot/>
		</section>
	</div>
</div>



<style>
	
</style>
