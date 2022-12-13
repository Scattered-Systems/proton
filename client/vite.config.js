import { sveltekit } from '@sveltejs/kit/vite';
import wasmPack from 'vite-plugin-wasm-pack';

const config = {
	plugins: [
		sveltekit(),
		wasmPack('../proton')
	]
};

export default config;
