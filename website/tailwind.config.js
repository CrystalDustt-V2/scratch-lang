/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    './src/pages/**/*.{js,jsx}',
    './src/components/**/*.{js,jsx}',
    './src/app/**/*.{js,jsx}',
    './src/**/*.{js,jsx}',
  ],
  theme: {
    extend: {
      colors: {
        scratch: {
          motion: '#4C97FF',
          looks: '#9966FF',
          sound: '#CF63CF',
          events: '#FFBF00',
          control: '#FFAB19',
          sensing: '#5CB1D6',
          operators: '#59C059',
          variables: '#FF8C1A',
          lists: '#FF661A',
          procedures: '#FF6680',
          music: '#0FBD8C',
          pen: '#0FBD8C',
          tts: '#0FBD8C',
          translate: '#0FBD8C',
          makey: '#0FBD8C',
        },
      },
    },
  },
  plugins: [],
};
