/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["index.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        /** From template beginning: */
        "logo-gray": "#444444",
        "logo-light-green": "#b3e1ce",
        "logo-dark-green": "#009a5b",
        /** From template end */
      },
      keyframes: {
        /** From template beginning: */
        moveDown: {
          "0%": { transform: "translateY(-50px)" },
          "100%": { transform: "translateY(0)" },
        },
        moveUp: {
          "0%": { transform: "translateY(50px)" },
          "100%": { transform: "translateY(0)" },
        },
        backgroundChange: {
          "0%": { backgroundColor: "black" },
          "100%": { backgroundColor: "white" },
        },
        /** From template end */
      },
      animation: {
        /** From template beginning: */
        "spin-slow": "spin 3s linear infinite",
        "move-down-once": "moveDown 1s ease-in-out forwards",
        "move-up-once": "moveUp 1s ease-in-out forwards",
        "background-change": "backgroundChange 1s ease-in-out forwards",
        /** From template end */
      },
    },
  },
  plugins: [],
};
