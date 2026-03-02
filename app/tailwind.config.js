/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'brand': {
          '50': '#f5f7fa',
          '100': '#e4e9f0',
          '200': '#ccd6e4',
          '300': '#a6bad1',
          '400': '#7a97ba',
          '500': '#5b7aa4',
          '600': '#48618a',
          '700': '#3b4e70',
          '800': '#34435d',
          '900': '#2f394f',
          '950': '#1f2533',
        },
        'celestial': {
          'purple': '#6366f1',
          'gold': '#f59e0b',
          'cyan': '#06b6d4',
        }
      },
      animation: {
        'shot-star': 'star 3s linear infinite',
        'fade-in': 'fadeIn 0.5s ease-out forwards',
      },
      keyframes: {
        star: {
          '0%': { transform: 'translateX(0) translateY(0) scale(0)', opacity: 0 },
          '10%': { opacity: 1 },
          '100%': { transform: 'translateX(300px) translateY(300px) scale(1)', opacity: 0 },
        },
        fadeIn: {
          'from': { opacity: 0, transform: 'translateY(10px)' },
          'to': { opacity: 1, transform: 'translateY(0)' },
        }
      }
    },
  },
  plugins: [],
}
