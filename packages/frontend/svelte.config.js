import adapter from '@sveltejs/adapter-cloudflare';
import preprocess from "svelte-preprocess";


const config = {
	kit: {
		adapter: adapter()
	},
	preprocess: [
		preprocess({
			postcss: true,
		}),
	],
};

export default config;
