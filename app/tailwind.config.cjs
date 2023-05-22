/** @type {import('tailwindcss').Config} */
const config = {
    content: [
        "./node_modules/flowbite/**/*.js",
        "./dist/**/*.{css,html}",
        "./src/**/*.{css,html,rs}",
        "./static/**/*.{css,html}"
    ],
    darkMode: 'class', // or 'media' or 'class'
    mode: 'jit',
    plugins: [
        require('flowbite/plugin')
    ],
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    },
};

module.exports = config;