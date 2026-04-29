<script setup>
import { reactive } from 'vue'
import { useVuelidate } from '@vuelidate/core'
import { required, minLength, helpers } from '@vuelidate/validators'

// 1. Khai báo state lưu trữ dữ liệu form đăng ký
const formData = reactive({
  fullName: '',
  emailOrPhone: '',
  password: ''
})

// 2. Định nghĩa các quy tắc kiểm tra (Rules)
const rules = {
  fullName: { 
    required: helpers.withMessage('Vui lòng nhập Họ và Tên', required) 
  },
  emailOrPhone: { 
    required: helpers.withMessage('Vui lòng nhập Email hoặc Số điện thoại', required) 
  },
  password: { 
    required: helpers.withMessage('Vui lòng nhập Mật khẩu', required),
    minLength: helpers.withMessage('Mật khẩu phải có ít nhất 6 ký tự', minLength(6))
  }
}

// 3. Khởi tạo Vuelidate
const v$ = useVuelidate(rules, formData)

// 4. Hàm xử lý khi bấm nút Tạo tài khoản
const handleRegister = async () => {
  // Kích hoạt kiểm tra toàn bộ form
  const isFormValid = await v$.value.$validate()
  
  if (!isFormValid) {
    console.log('Form không hợp lệ, vui lòng kiểm tra lại!')
    return
  }

  // Nếu hợp lệ, tiến hành gọi API backend ở đây
  console.log('Dữ liệu hợp lệ, sẵn sàng gửi API tạo User:', formData)
  // TODO: Gọi service API để tạo user trong database qua Rust Axum
}
</script>

<template>
  <div class="container mx-auto px-4 py-8">
    <div class="flex flex-col md:flex-row bg-white rounded-lg shadow-sm border overflow-hidden min-h-[600px] max-w-5xl mx-auto">
      
      <!-- Cột Trái: Hình ảnh -->
      <div class="md:w-1/2 bg-[#e8f3f8] relative flex items-center justify-center p-8">
        <div class="text-center text-gray-400 border-2 border-dashed border-gray-300 w-full h-full flex items-center justify-center rounded">
          <span>[Hình ảnh Điện thoại & Giỏ hàng]</span>
        </div>
      </div>

      <!-- Cột Phải: Form Đăng ký -->
      <div class="md:w-1/2 p-8 md:p-12 flex flex-col justify-center">
        <h2 class="text-3xl font-bold mb-2 text-gray-900">Tạo tài khoản</h2>
        <p class="text-gray-500 mb-8 text-sm">Điền thông tin</p>
        
        <form @submit.prevent="handleRegister">
          
          <!-- Input Họ và Tên -->
          <div class="mb-6 relative">
            <input 
              type="text" 
              v-model="formData.fullName"
              placeholder="Họ và Tên" 
              :class="{ 'border-danger': v$.fullName.$error }"
              class="w-full border-b border-gray-300 py-3 focus:outline-none focus:border-primary transition-colors bg-transparent placeholder-gray-400 text-sm" 
            />
            <span v-if="v$.fullName.$error" class="text-danger text-xs absolute -bottom-4 left-0">
              {{ v$.fullName.$errors[0].$message }}
            </span>
          </div>

          <!-- Input Email/Phone -->
          <div class="mb-6 relative mt-6">
            <input 
              type="text" 
              v-model="formData.emailOrPhone"
              placeholder="Email hoặc Số Điện Thoại" 
              :class="{ 'border-danger': v$.emailOrPhone.$error }"
              class="w-full border-b border-gray-300 py-3 focus:outline-none focus:border-primary transition-colors bg-transparent placeholder-gray-400 text-sm" 
            />
            <span v-if="v$.emailOrPhone.$error" class="text-danger text-xs absolute -bottom-4 left-0">
              {{ v$.emailOrPhone.$errors[0].$message }}
            </span>
          </div>

          <!-- Input Password -->
          <div class="mb-8 relative mt-6">
            <input 
              type="password" 
              v-model="formData.password"
              placeholder="Mật khẩu" 
              :class="{ 'border-danger': v$.password.$error }"
              class="w-full border-b border-gray-300 py-3 focus:outline-none focus:border-primary transition-colors bg-transparent placeholder-gray-400 text-sm" 
            />
            <span v-if="v$.password.$error" class="text-danger text-xs absolute -bottom-4 left-0">
              {{ v$.password.$errors[0].$message }}
            </span>
          </div>
          
          <button 
            type="submit"
            class="w-full bg-primary text-white py-3 rounded hover:bg-blue-600 transition font-medium mt-2">
            Tạo tài khoản
          </button>
          
          <!-- Nút Google -->
          <button 
            type="button"
            class="w-full border border-gray-300 text-gray-700 py-3 rounded mt-4 flex items-center justify-center gap-2 hover:bg-gray-50 transition font-medium">
            <svg class="w-5 h-5" viewBox="0 0 24 24">
              <path fill="#4285F4" d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"/>
              <path fill="#34A853" d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"/>
              <path fill="#FBBC05" d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"/>
              <path fill="#EA4335" d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"/>
            </svg>
            Sign up with Google
          </button>
          
          <p class="text-center mt-6 text-gray-500 text-sm">
            Bạn đã có tài khoản? 
            <router-link to="/login" class="text-gray-900 font-medium hover:underline border-b border-gray-900 pb-0.5">
              Đăng nhập
            </router-link> 
          </p>
        </form>
      </div>

    </div>
  </div>
</template>