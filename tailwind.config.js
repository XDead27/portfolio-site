// tailwind.config.js
const colors = require('tailwindcss/colors');

module.exports = {
  theme: {
    extend: {
      colors: {
        wm: {
          bg: '#14080E',
          cyan: '#00ffff',
        },
        green: {
          DEFAULT: '#3A5743',
          100: '#233428',
          200: '#2D4334',
          300: '#3A5743',
          500: '#00ff00',
        },
      },
      fontFamily: {
        mono: ['Fira Code', 'monospace'],
      },
    },
  },
  content: ['./src/**/*.{rs,html}'],
};
