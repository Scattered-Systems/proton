module.exports = {
    content: [
        './src/**/*.rs',
        './public/index.html',
        './public/**/*.html',
        './public/**/*.css'
    ],
    darkMode: 'class',
    mode: 'jit',
    plugins: [],
    purge: [
        "src/**/*.rs"
    ],
    theme: {
        extend: {},
    },
    variants: {
        extend: {},
    }
}
