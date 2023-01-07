import adapter from '@sveltejs/adapter-auto';
import * as noded from '@sveltejs/adapter-node';
import preprocess from "svelte-preprocess";


const config = {
	kit: {
		...(process.env.MODE === "production") && {
			adapter: noded()
		},
		...(process.env.MODE !== "production") && {
			adapter: adapter()
		}
	},
	preprocess: [
		preprocess({
			postcss: true,
		}),
	],
};

export default config;
