import { resolve } from 'node:path'
import { defineConfig } from 'vite'

export default defineConfig({
  build: {
    rollupOptions: {
      input: {
        index: resolve(__dirname, 'index.html'),
        basic: resolve(__dirname, 'basic/index.html'),
        light: resolve(__dirname, 'light/index.html'),
        texture: resolve(__dirname, 'texture/index.html'),
      },
    },
  },
})
