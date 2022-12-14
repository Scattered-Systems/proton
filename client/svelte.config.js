// import adapter from '@sveltejs/adapter-auto';
import * as cloudflare from '@sveltejs/adapter-cloudflare';
import adapter from '@sveltejs/adapter-node';
import * as vercel from '@sveltejs/adapter-vercel';
import preprocess from "svelte-preprocess";


const config = {
	kit: {
		...(process.env.MODE === "cloudflare") && {
			adapter: cloudflare()
		},
		...(process.env.MODE === "vercel") && {
			adapter: vercel()
		},
		...(process.env.MODE !== "vercel" && process.env.MODE !=="cloudflare") && {
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
