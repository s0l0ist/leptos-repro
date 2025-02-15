/** @type {import('tailwindcss').Config} */
import forms from '@tailwindcss/forms';

export default {
  content: {
    relative: true,
    files: ["*.html", "./app/**/*.rs"],
  },
  theme: {
    extend: {
      backgroundImage: {
        /**
         * Unfortunately, this is the best way to style the native <select> html elements.
         * 
         * Use this site to convert to base64: https://www.fffuel.co/eeencode/
         * 
         * Input SVG:
         * <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="#9CA3AF">
         *   <path fill-rule="evenodd" d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z" clip-rule="evenodd" />
         * </svg>
         * 
         */
        chevron: `url("data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyMCAyMCIgZmlsbD0iIzlDQTNBRiI+CiAgPHBhdGggZmlsbC1ydWxlPSJldmVub2RkIiBkPSJNNS4yMiA4LjIyYS43NS43NSAwIDAgMSAxLjA2IDBMMTAgMTEuOTRsMy43Mi0zLjcyYS43NS43NSAwIDEgMSAxLjA2IDEuMDZsLTQuMjUgNC4yNWEuNzUuNzUgMCAwIDEtMS4wNiAwTDUuMjIgOS4yOGEuNzUuNzUgMCAwIDEgMC0xLjA2WiIgY2xpcC1ydWxlPSJldmVub2RkIiAvPgo8L3N2Zz4=")`,
      }
    },
  },
  plugins: [forms],
}
