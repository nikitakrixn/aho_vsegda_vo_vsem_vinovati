/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./components/**/*.{js,vue,ts}",
    "./layouts/**/*.vue",
    "./pages/**/*.vue",
    "./plugins/**/*.{js,ts}",
    "./app.vue",
    "./error.vue",
  ],
  theme: {
    extend: {
      fontFamily: {
        onest: ['Onest', 'sans-serif'],
      },
      colors: {
        primary: '#4a7dff',
        success: '#51ce2c',
        warning: '#ff8800',
        error: '#f93939',
        gray: '#656b7b',
        light: '#f2f3f7',
      },
    },
  },
  plugins: [],
}

