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
    fontFamily: {
      basementGrotesque: ["BasementGrotesque", "sans-serif"],
      martianMono: ["MartianMono", "sans-serif"],
    },
    extend: {
      colors: {
        light1: "var(--light1)",
        light2: "var(--light2)",
        light3: "var(--light3)",
        lightcolor: "var(--lightcolor)",
        dark1: "var(--dark1)",
        dark2: "var(--dark2)",
        dark3: "var(--dark3)",
        darkcolor: "var(--darkcolor)",
      },
    },
  },
  plugins: [],
};
