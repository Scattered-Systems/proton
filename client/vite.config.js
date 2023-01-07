import { sveltekit } from '@sveltejs/kit/vite';
import wasmPack from 'vite-plugin-wasm-pack';

const config = {
	plugins: [
		sveltekit(),
		wasmPack('./wasm')
	]
};

export default config;
