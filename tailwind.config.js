/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",   // Scans all Rust files in src
    "./index.html",    // Scans your root HTML
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

