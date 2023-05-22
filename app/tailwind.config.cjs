/** @type {import('tailwindcss').Config} */
const config = {
    content: [
        "./dist/**/*.{css,html}",
        "./src/**/*.{css,html,rs}",
        "./static/**/*.{css,html}"
    ],
    darkMode: 'class', // or 'media' or 'class'
    mode: 'jit',
    plugins: [],
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    },
};

module.exports = config;