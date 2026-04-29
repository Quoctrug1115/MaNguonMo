import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      // Dòng này báo cho Vite biết: hễ gặp chữ '@' thì trỏ thẳng vào thư mục 'src'
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  }
})