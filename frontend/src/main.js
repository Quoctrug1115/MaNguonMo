import { createApp } from 'vue'
import './style.css' // File chứa cấu hình Tailwind CSS của chúng ta
import App from './App.vue'
import router from './router' // Import file router cấu hình ở Bước 2
import vue3GoogleLogin from 'vue3-google-login'

const app = createApp(App)

app.use(router) // Đăng ký router

app.use(vue3GoogleLogin, {
    clientId: '602877997229-ruc9269uqqatmtsm2npt4q2p7e8s7r0d.apps.googleusercontent.com'
})

app.mount('#app')