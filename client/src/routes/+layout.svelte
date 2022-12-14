<script>
	import '$lib/assets/css/styles.css';
	import { Navbar, Sidebar, Toolbar, WalletToggle } from '$lib';

	let open = false;
	let linktree = [ 
		{ href: '/editor', label: 'Editor'},
		{ href: '/settings', label: 'Settings'}
	]
</script>

<div class="bg-gradient-to-br from-zinc-900 via-zinc-800 to-zinc-900 flex flex-col items-center justify-center m-0 p-0 z-0 min-h-screen min-w-full max-w-screen" id="base">
	<Navbar name={"Puzzled"}>
		<WalletToggle/>
	</Navbar>
	<div class="flex flex-col grow items-center justify-center min-h-full max-h-screen min-w-full max-w-screen z-0" id="content-section">
		<main class="bg-zinc-700 flex container mx-auto prose prose-invert rounded p-3">
			<slot/>
		</main>
	</div>
	<Toolbar 
		linktree={linktree} 
		bind:sidebar={open}
	/>
</div>

<Sidebar bind:open>
	Control Center
</Sidebar>


<style>
	
</style>
