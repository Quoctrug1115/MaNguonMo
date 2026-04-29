<template>
  <div class="container mx-auto px-4 py-8">
    <div class="flex flex-col md:flex-row bg-white rounded-lg shadow-sm border overflow-hidden min-h-[600px] max-w-5xl mx-auto">
      
      <div class="md:w-1/2 bg-[#e8f3f8] relative flex items-center justify-center p-8">
        <div class="text-center text-gray-400 border-2 border-dashed border-gray-300 w-full h-full flex items-center justify-center rounded">
          <span>[Hình ảnh Điện thoại & Giỏ hàng]</span>
        </div>
      </div>

      <div class="md:w-1/2 p-8 md:p-16 flex flex-col justify-center">
        <h2 class="text-3xl font-bold mb-2 text-gray-900">Quên mật khẩu</h2>
        <p class="text-gray-500 mb-10 text-sm">Nhập email để nhận liên kết đặt lại mật khẩu</p>
        
        <form @submit.prevent>
          <div class="mb-8">
            <input 
              type="email" 
              placeholder="Email của bạn" 
              class="w-full border-b border-gray-300 py-3 focus:outline-none focus:border-primary transition-colors bg-transparent placeholder-gray-400 text-sm" 
            />
          </div>
          
          <button class="w-full bg-primary text-white py-3 rounded hover:bg-blue-600 transition font-medium">
            Gửi liên kết
          </button>
          
          <p class="text-center mt-8 text-sm">
            <router-link to="/login" class="text-gray-500 hover:text-primary transition">
              ← Quay lại đăng nhập
            </router-link>
          </p>
        </form>
      </div>

    </div>
  </div>
</template>