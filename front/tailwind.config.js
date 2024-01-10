/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        mc: ['Minecraftia', 'sans-serif'],
        tinybox: ['Tiny Box BlackBitA8', 'sans-serif'],
      },
    },
  },
  plugins: [],
}

