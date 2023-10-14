import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  base: "./",
  plugins: [
    react(),
  ],
  server: {
    host: "0.0.0.0",
    port: 8080
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, 'src')
    }
  },
  build: {
    rollupOptions: {
      input: {
        // popup 弹窗入口
        popup: "popup.html",

        // options 选项配置入口
        options: "options.html"
      }
    }
  }
})
