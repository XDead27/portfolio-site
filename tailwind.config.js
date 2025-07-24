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
        purple: {
          DEFAULT: '#5C354B',
          50 : '#F7F2F6',
          100: '#F0E6ED',
          200: '#D6C1D0',
          300: '#BD9DB3',
          400: '#8C647D',
          500: '#5C354B',
          600: '#522A3F',
          700: '#451E31',
          800: '#381424',
          900: '#290B17',
          950: '#1A040C',
        },
        green: {
          DEFAULT: '#3A5743',
          100: '#233428',
          200: '#2D4334',
          300: '#3A5743',
          500: '#00ff00',
        },
        beige: {
          DEFAULT: '#CEC288',
        },
      },
      fontFamily: {
        mono: ['Fira Code', 'monospace'],
      },
    },
  },
  content: ['./src/**/*.{rs,html}'],
};
