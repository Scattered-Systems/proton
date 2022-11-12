import { sveltekit } from '@sveltejs/kit/vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

const config = {
	plugins: [
		sveltekit(),
		topLevelAwait(),
		wasm()
	]
};

export default config;
