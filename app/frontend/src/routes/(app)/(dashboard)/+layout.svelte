<script>
</script>

<div class="grid grid-flow-row-dense grid-cols-4 h-full z-10">
	<div class="col-span-4 md:col-span-1 h-full">
		<section class="flex flex-auto grow items-center justify-center p-3 h-full">
			<slot/>
		</section>
	</div>
	<div class="md:col-span-1 hidden">
		<div class="flex flex-initial mx-auto p-3">
			<div class="bg-zinc-800 flex flex-col rounded p-3 wrap items-center justify-center">
				<span>Feed</span>
			</div>
		</div>
	</div>
</div>



<style>
	
</style>
