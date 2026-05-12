<script setup>
import { GoogleLogin } from 'vue3-google-login'
import axios from 'axios'
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// 1. Biến chứa dữ liệu form đăng nhập
const form = reactive({
  email: '',
  password: ''
})

const errorMessage = ref('')

// 2. Hàm xử lý Đăng nhập truyền thống
const handleLogin = async () => {
  errorMessage.value = ''
  if (!form.email || !form.password) {
    alert('Vui lòng nhập email và mật khẩu!')
    return
  }

  try {
    const response = await fetch('http://localhost:3000/api/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        email: form.email,
        password: form.password
      })
    })

    const data = await response.json()

    if (response.ok) {
      alert(data.message)

      // Lấy role (Đề phòng backend trả null, ta dự phòng là 'user')
      const userRole = data.user.role || 'user'

      // Lưu Token, User và Role vào LocalStorage
      localStorage.setItem('token', data.token)
      localStorage.setItem('user', JSON.stringify(data.user))
      localStorage.setItem('role', userRole) // Vệ sĩ Router sẽ dùng cái này để kiểm tra

      // PHÂN LUỒNG CHUYỂN HƯỚNG
      alert('Đăng nhập thành công!')

        router.push('/')

    } else {
      errorMessage.value = data.error || 'Đăng nhập thất bại'
      alert(errorMessage.value)
    }

  } catch (error) {
    console.error('Lỗi kết nối API:', error)
    alert('Không thể kết nối đến máy chủ Backend.')
  }
}

// 3. Hàm xử lý Đăng nhập Google
const handleGoogleLoginSuccess = async (response) => {
  const token = response.credential;
  
  try {
    const res = await axios.post('http://localhost:3000/api/auth/google', {
      token: token
    });
    
    const userData = res.data.user;
    const userRole = userData.role || 'user';

    // Lưu thông tin vào bộ nhớ trình duyệt
    localStorage.setItem('token', res.data.token);
    localStorage.setItem('user', JSON.stringify(userData)); 
    localStorage.setItem('role', userRole); 
    
    alert('Đăng nhập Google thành công!');

      router.push('/')
      
  } catch (error) {
    console.error("Lỗi đăng nhập:", error);
    alert("Có lỗi xảy ra khi đăng nhập Google!");
  }
}
</script>

<template>
  <div class="flex flex-col lg:flex-row items-center mt-10 mb-24 max-w-7xl mx-auto overflow-hidden">

    <!-- Cột TRÁI: Hình ảnh minh họa -->
    <div class="hidden lg:block lg:w-1/2 pr-10">
      <div class="bg-[#cbe4e8] h-[700px] flex items-center justify-center rounded-r-lg overflow-hidden">
        <div class="w-full h-full flex flex-col items-center justify-center text-teal-700 opacity-70">
          <svg class="w-24 h-24 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>
          <span class="font-medium text-lg">[Hình ảnh Điện thoại & Giỏ hàng]</span>
        </div>
      </div>
    </div>

    <!-- Cột PHẢI: Form Đăng nhập -->
    <div class="w-full lg:w-1/2 px-6 sm:px-12 md:px-24 flex flex-col justify-center">

      <h1 class="text-3xl md:text-4xl font-medium text-gray-900 mb-4 tracking-tight">Đăng nhập</h1>
      <p class="text-base text-gray-600 mb-12">Nhập thông tin chi tiết dưới đây</p>

      <form @submit.prevent="handleLogin" class="space-y-10">

        <!-- Input Email -->
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
        <div class="flex items-center justify-between pt-4">

          <!-- Nút Đăng nhập -->
          <button
              type="submit"
              class="bg-primary text-white py-4 px-10 rounded text-base font-medium hover:bg-blue-600 transition"
          >
            Đăng nhập
          </button>
          
          <!-- Quên mật khẩu -->
          <router-link to="/forgot-password" class="text-danger hover:underline transition font-medium">
            Quên mật khẩu?
          </router-link>
          
        </div>
        
      </form>
      <!-- Nút Đăng nhập Google-->
      <div class="mt-8">
        <div class="flex justify-center">
          <GoogleLogin :callback="handleGoogleLoginSuccess" />
        </div>
      </div>

      <!-- Liên kết sang Đăng ký -->
      <div class="mt-8 text-center text-base text-gray-600 flex items-center justify-center gap-2">
        <span>Bạn chưa có tài khoản?</span>
        <router-link to="/register" class="text-gray-900 hover:text-primary font-medium underline transition pb-0.5 border-b border-transparent hover:border-primary">
          Đăng ký ngay
        </router-link>
      </div>

    </div>

  </div>
</template>
