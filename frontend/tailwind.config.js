/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/*'],
  theme: {
    extend: {
      textColor: theme => ({
        ...theme("colors"),
         // 强调/正文标题
         'color-1': '#1D2129',
         // 次强调/正文标题
         'color-2': '#4E5969',
         // 次要信息
         'color-3': '#86909C',
         // 置灰信息
         'color-4': '#CCCCCC',
      })
    },
  },
  plugins: [],
}

