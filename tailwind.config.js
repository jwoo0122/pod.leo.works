/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        title: ["Silkscreen", "san-serif"],
      },
    },
  },
  plugins: [],
};
