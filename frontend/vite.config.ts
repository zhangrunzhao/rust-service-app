import { defineConfig } from 'vite';
import tailwindcss from 'tailwindcss';
import autoprefixer from 'autoprefixer';
import react from '@vitejs/plugin-react-swc';
import { vitePluginForArco } from '@arco-plugins/vite-react';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), vitePluginForArco()],

  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },

  css: {
    postcss: {
      plugins: [tailwindcss, autoprefixer],
    },
  },
});
