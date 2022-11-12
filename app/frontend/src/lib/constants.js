import { Navbar } from './nav/Navbar.svelte';

export let navbar = Navbar;

export let info = {
    homepage: "https://scattered-systems.com",

    name: "Portal",
    slug: "portal",
    pages: [
        {
            href: "/",
            label: "Home"
        }
    ]
}