<script setup>
import { reactive } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// Biến chứa dữ liệu form
const form = reactive({
  name: '',
  email: '',
  password: ''
})

// Hàm xử lý đăng ký gọi API Rust
const handleRegister = async () => {
  if (!form.name || !form.email || !form.password) {
    alert('Vui lòng điền đầy đủ thông tin!')
    return
  }

  try {
    const response = await fetch(`${import.meta.env.VITE_API_BASE_URL}/register`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        full_name: form.name,
        email: form.email,
        password: form.password
      })
    })

    const data = await response.json()

    if (response.ok) {
      alert(data.message)
      router.push('/login') // Đẩy sang trang đăng nhập
    } else {
      alert(data.error)
    }

  } catch (error) {
    console.error('Lỗi khi gọi API:', error)
    alert('Không thể kết nối đến máy chủ Backend. Vui lòng kiểm tra lại server Rust.')
  }
}
</script>

<template>
  <!-- Container chính bọc toàn bộ khối 2 cột -->
  <div class="flex flex-col lg:flex-row items-center mt-10 mb-24 max-w-7xl mx-auto overflow-hidden">

    <!-- Cột TRÁI: Hình ảnh minh họa (Ẩn trên mobile, hiện trên màn hình lớn) -->
    <div class="hidden lg:block lg:w-1/2 pr-10">
      <div class="bg-[#cbe4e8] h-[700px] flex items-center justify-center rounded-r-lg overflow-hidden">
        <!-- Sau này bạn thay đường dẫn ảnh thật vào thẻ src này -->
        <div class="w-full h-full flex flex-col items-center justify-center text-teal-700 opacity-70">
          <svg class="w-24 h-24 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>
          <span class="font-medium text-lg">[Hình ảnh Điện thoại & Giỏ hàng]</span>
        </div>
      </div>
    </div>

    <!-- Cột PHẢI: Form Đăng ký -->
    <div class="w-full lg:w-1/2 px-6 sm:px-12 md:px-24 flex flex-col justify-center">

      <h1 class="text-3xl md:text-4xl font-medium text-gray-900 mb-4 tracking-tight">Tạo tài khoản</h1>
      <p class="text-base text-gray-600 mb-12">Điền thông tin</p>

      <!-- Form gọi hàm handleRegister -->
      <form @submit.prevent="handleRegister" class="space-y-10">

        <!-- Input Tên -->
        <div>
          <input
              type="text"
              v-model="form.name"
              placeholder="Họ và Tên"
              required
              class="w-full border-b border-gray-300 py-2 focus:outline-none focus:border-primary transition-colors text-gray-700 text-base bg-transparent placeholder-gray-400"
          />
        </div>

        <!-- Input Email/Phone -->
        <div>
          <input
              type="email"
              v-model="form.email"
              placeholder="Email hoặc Số Điện Thoại"
              required
              class="w-full border-b border-gray-300 py-2 focus:outline-none focus:border-primary transition-colors text-gray-700 text-base bg-transparent placeholder-gray-400"
          />
        </div>

        <!-- Input Mật khẩu -->
        <div>
          <input
              type="password"
              v-model="form.password"
              placeholder="Mật khẩu"
              required
              class="w-full border-b border-gray-300 py-2 focus:outline-none focus:border-primary transition-colors text-gray-700 text-base bg-transparent placeholder-gray-400"
          />
        </div>

        <!-- Khối Nút Bấm -->
        <div class="flex flex-col gap-4 pt-4">

          <!-- Nút Đăng ký (Xanh dương) -->
          <button
              type="submit"
              class="w-full bg-primary text-white py-4 rounded text-base font-medium hover:bg-blue-600 transition"
          >
            Tạo tài khoản
          </button>

          <!-- Nút Sign up with Google -->
          <button
              type="button"
              class="w-full bg-white border border-gray-300 text-gray-700 py-4 rounded text-base font-medium hover:bg-gray-50 transition flex items-center justify-center gap-3"
          >
            <!-- Biểu tượng Google SVG -->
            <svg class="w-5 h-5" viewBox="0 0 24 24">
              <path d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z" fill="#4285F4"/>
              <path d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z" fill="#34A853"/>
              <path d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z" fill="#FBBC05"/>
              <path d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z" fill="#EA4335"/>
            </svg>
            Sign up with Google
          </button>

        </div>

      </form>

      <!-- Liên kết sang Đăng nhập -->
      <div class="mt-8 text-center text-base text-gray-600 flex items-center justify-center gap-2">
        <span>Đã có tài khoản?</span>
        <router-link to="/login" class="text-gray-900 hover:text-primary font-medium underline transition pb-0.5 border-b border-transparent hover:border-primary">
          Đăng nhập
        </router-link>
      </div>

    </div>

  </div>
</template>