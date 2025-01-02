/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs", "./src/*.rs"],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
        },
    },
    theme: {
        colors: {
            'text': 'var(--text)',
            'background': 'var(--background)',
            'primary': 'var(--primary)',
            'secondary': 'var(--secondary)',
            'accent': 'var(--accent)',
        },

        extend: {},
    },
    plugins: [],
}
