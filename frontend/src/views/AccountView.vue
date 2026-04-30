<script setup>
import { reactive } from 'vue'

// Dữ liệu giả lập cho form (Sau này sẽ gọi API GET /api/user/profile từ Rust Axum để điền vào đây)
const userProfile = reactive({
  firstName: 'Trung', 
  lastName: 'Nguyễn',
  email: 'trung.nguyen@gmail.com',
  address: 'Kingston, 5236, United State',
  currentPassword: '',
  newPassword: '',
  confirmPassword: ''
})

const handleSaveProfile = () => {
  console.log('Dữ liệu cập nhật:', userProfile)
  // TODO: Gọi API PUT hoặc PATCH lên server để lưu thay đổi
  // axios.put('/api/user/profile', userProfile)
}
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20">
    
    <!-- Top Area: Breadcrumb & Greeting -->
    <div class="flex justify-between items-center mb-16">
      <div class="text-sm text-gray-500">
        <router-link to="/" class="hover:text-primary transition">Home</router-link> / 
        <span class="text-gray-900">My Account</span>
      </div>
      <div class="text-sm">
        Xin Chào! <span class="text-danger font-medium">{{ userProfile.firstName }}</span>
      </div>
    </div>

    <div class="flex flex-col md:flex-row gap-10">
      
      <!-- CỘT TRÁI: SIDEBAR ĐIỀU HƯỚNG -->
      <div class="md:w-1/4">
        
        <!-- Nhóm 1: Quản lý tài khoản -->
        <div class="mb-6">
          <h3 class="font-medium text-gray-900 mb-4">Quản lý tài khoản</h3>
          <ul class="flex flex-col gap-3 pl-6 text-sm">
            <li class="text-danger font-medium cursor-pointer">Tài khoản</li>
            <li class="text-gray-500 hover:text-primary transition cursor-pointer">Địa chỉ nhận hàng</li>
            <li class="text-gray-500 hover:text-primary transition cursor-pointer">Phương thức thanh toán</li>
          </ul>
        </div>

        <!-- Nhóm 2: Đơn hàng của tôi -->
        <div class="mb-6">
          <h3 class="font-medium text-gray-900 mb-4">Đơn hàng của tôi</h3>
          <ul class="flex flex-col gap-3 pl-6 text-sm">
            <li class="text-gray-500 hover:text-primary transition cursor-pointer">Hoàn hàng</li>
            <li class="text-gray-500 hover:text-primary transition cursor-pointer">Hủy đơn hàng</li>
          </ul>
        </div>

        <!-- Nhóm 3: Wishlist -->
        <div>
          <h3 class="font-medium text-gray-900 hover:text-primary transition cursor-pointer">
            <router-link to="/wishlist">Yêu thích</router-link>
          </h3>
        </div>

      </div>

      <!-- CỘT PHẢI: FORM CHỈNH SỬA THÔNG TIN -->
      <div class="md:w-3/4 bg-white shadow-sm border border-gray-100 rounded-lg p-8 lg:p-14">
        <h2 class="text-danger text-xl font-medium mb-8">Chỉnh sửa</h2>
        
        <form @submit.prevent="handleSaveProfile">
          
          <!-- Hàng 1: Họ & Tên -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
            <div>
              <label class="block text-gray-900 text-sm mb-2 font-medium">Họ</label>
              <input type="text" v-model="userProfile.firstName" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm text-gray-700" />
            </div>
            <div>
              <label class="block text-gray-900 text-sm mb-2 font-medium">Tên</label>
              <input type="text" v-model="userProfile.lastName" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm text-gray-700" />
            </div>
          </div>

          <!-- Hàng 2: Email & Địa chỉ -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
            <div>
              <label class="block text-gray-900 text-sm mb-2 font-medium">Email</label>
              <input type="email" v-model="userProfile.email" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm text-gray-700" />
            </div>
            <div>
              <label class="block text-gray-900 text-sm mb-2 font-medium">Địa chỉ</label>
              <input type="text" v-model="userProfile.address" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm text-gray-700" />
            </div>
          </div>

          <!-- Hàng 3: Đổi Mật Khẩu -->
          <div class="space-y-6">
            <label class="block text-gray-900 text-sm mb-2 font-medium">Mật Khẩu</label>
            <input type="password" v-model="userProfile.currentPassword" placeholder="mật khẩu cũ" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm placeholder-gray-400" />
            <input type="password" v-model="userProfile.newPassword" placeholder="mật khẩu mới" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm placeholder-gray-400" />
            <input type="password" v-model="userProfile.confirmPassword" placeholder="nhập lại mật khẩu mới" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm placeholder-gray-400" />
          </div>

          <!-- Nút hành động -->
          <div class="flex justify-end items-center gap-8 mt-10">
            <button type="button" class="text-gray-600 hover:text-gray-900 font-medium transition">
              Thoát
            </button>
            <button type="submit" class="bg-primary text-white px-10 py-3 rounded font-medium hover:bg-blue-600 transition">
              Lưu
            </button>
          </div>

        </form>
      </div>

    </div>
  </div>
</template>