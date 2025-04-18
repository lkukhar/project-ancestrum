import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': resolve(__dirname, './app'),
      'store': resolve(__dirname, './app/store'),
      'services': resolve(__dirname, './app/services'),
      'components': resolve(__dirname, './app/components')
    }
  },
  envPrefix: ['VITE_', 'TAURI_'],
  clearScreen: false,
  server: {
    port: 3000,
    strictPort: true,
  },
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  }
}) 