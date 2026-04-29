import { createApp } from 'vue'
import './style.css' // File chứa cấu hình Tailwind CSS của chúng ta
import App from './App.vue'
import router from './router' // Import file router cấu hình ở Bước 2

const app = createApp(App)

app.use(router) // Đăng ký router
app.mount('#app')