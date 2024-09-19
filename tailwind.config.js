/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {

    fontFamily: {
      sans: ["Montserrat", "sans-serif"]
    },
    extend: {
      colors: {
        accent: '#97D8C4',
        'accent-dark': '#1a926d'
      }
    },
  },
  plugins: [],
}
