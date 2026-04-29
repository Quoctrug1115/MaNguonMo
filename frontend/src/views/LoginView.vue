<script setup>
import { reactive } from 'vue'
import { useVuelidate } from '@vuelidate/core'
import { required, minLength, helpers } from '@vuelidate/validators'

// 1. Khai báo state lưu trữ dữ liệu form
const formData = reactive({
  emailOrPhone: '',
  password: ''
})

// 2. Định nghĩa các quy tắc kiểm tra (Rules)
const rules = {
  emailOrPhone: { 
    // Tùy chỉnh thông báo lỗi bằng helpers.withMessage
    required: helpers.withMessage('Vui lòng nhập Email hoặc Số điện thoại', required) 
  },
  password: { 
    required: helpers.withMessage('Vui lòng nhập Mật khẩu', required),
    minLength: helpers.withMessage('Mật khẩu phải có ít nhất 6 ký tự', minLength(6))
  }
}

// 3. Khởi tạo Vuelidate
const v$ = useVuelidate(rules, formData)

// 4. Hàm xử lý khi bấm nút Đăng nhập
const handleLogin = async () => {
  // Kích hoạt kiểm tra toàn bộ form
  const isFormValid = await v$.value.$validate()
  
  if (!isFormValid) {
    console.log('Form không hợp lệ, vui lòng kiểm tra lại!')
    return
  }

  // Nếu hợp lệ, tiến hành gọi API backend ở đây
  console.log('Dữ liệu hợp lệ, chuẩn bị gửi lên server Axum:', formData)
  // TODO: Gọi service API...
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

      <!-- Cột Phải: Form Đăng nhập -->
      <div class="md:w-1/2 p-8 md:p-16 flex flex-col justify-center">
        <h2 class="text-3xl font-bold mb-2 text-gray-900">Đăng nhập</h2>
        <p class="text-gray-500 mb-10 text-sm">Điền thông tin của bạn</p>
        
        <form @submit.prevent="handleLogin">
          <!-- Input Email/Phone -->
          <div class="mb-6 relative">
            <input 
              type="text" 
              v-model="formData.emailOrPhone"
              placeholder="Email hoặc Số Điện Thoại" 
              :class="{ 'border-danger': v$.emailOrPhone.$error }"
              class="w-full border-b border-gray-300 py-3 focus:outline-none focus:border-primary transition-colors bg-transparent placeholder-gray-400 text-sm" 
            />
            <!-- Hiển thị lỗi -->
            <span v-if="v$.emailOrPhone.$error" class="text-danger text-xs absolute -bottom-4 left-0">
              {{ v$.emailOrPhone.$errors[0].$message }}
            </span>
          </div>

          <!-- Input Password -->
          <div class="mb-6 relative mt-6">
            <input 
              type="password" 
              v-model="formData.password"
              placeholder="Mật khẩu" 
              :class="{ 'border-danger': v$.password.$error }"
              class="w-full border-b border-gray-300 py-3 focus:outline-none focus:border-primary transition-colors bg-transparent placeholder-gray-400 text-sm" 
            />
            <!-- Hiển thị lỗi -->
            <span v-if="v$.password.$error" class="text-danger text-xs absolute -bottom-4 left-0">
              {{ v$.password.$errors[0].$message }}
            </span>
          </div>
          
          <div class="flex items-center justify-between mt-10">
            <button 
              type="submit"
              class="bg-primary text-white px-10 py-3 rounded hover:bg-blue-600 transition font-medium">
              Đăng nhập
            </button>
            <router-link to="/forgot-password" class="text-danger hover:underline text-sm">
              Quên Mật Khẩu ?
            </router-link>
          </div>

          <p class="text-center mt-8 text-gray-500 text-sm">
            Bạn chưa có tài khoản? 
            <router-link to="/register" class="text-primary font-medium hover:underline">
              Đăng ký ngay
            </router-link> 
          </p>
        </form>
      </div>

    </div>
  </div>
</template>