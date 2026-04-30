<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// Quản lý trạng thái mở/đóng của dropdown tài khoản
const isAccountMenuOpen = ref(false)

// Trạng thái đăng nhập giả lập
const isLoggedIn = ref(true) 
const userName = ref('Trung')

// Hàm xử lý khi bấm Đăng xuất
const handleLogout = () => {
  // 1. Chuyển trạng thái đăng nhập về false
  isLoggedIn.value = false
  
  // 2. Đóng menu dropdown
  isAccountMenuOpen.value = false
  
  // 3. (Tùy chọn) Chuyển hướng người dùng về trang chủ hoặc trang đăng nhập
  router.push('/login') 
}
</script>

<template>
  <header class="bg-white border-b border-gray-100 sticky top-0 z-50">
    <div class="container mx-auto px-4 py-4 flex items-center justify-between max-w-6xl">
      
      <!-- Logo -->
      <router-link to="/" class="text-2xl font-bold text-gray-900 tracking-tight">
        Cửa Hàng Điện Máy
      </router-link>

      <!-- Navigation Links -->
      <nav class="hidden md:flex items-center gap-8 text-sm font-medium text-gray-700">
        <router-link to="/" class="hover:text-primary transition">Trang chủ</router-link>
        <a href="#" class="hover:text-primary transition">Liên hệ</a>
        <a href="#" class="hover:text-primary transition">Về chúng tôi</a>
        
        <!-- Xử lý logic Đăng nhập / Hiển thị tên -->
        <router-link v-if="!isLoggedIn" to="/login" class="hover:text-primary transition">
          Đăng nhập
        </router-link>
        <div v-else class="text-primary font-medium cursor-pointer">
          Xin chào, {{ userName }}
        </div>
      </nav>

      <!-- Cụm chức năng góc phải -->
      <div class="flex items-center gap-5">
        
        <!-- Search Bar -->
        <div class="hidden lg:flex items-center relative mr-2">
          <input 
            type="text" 
            placeholder="Tìm kiếm điều bạn cần" 
            class="bg-[#f5f5f5] text-sm rounded-md pl-4 pr-10 py-2 focus:outline-none focus:ring-1 focus:ring-primary w-[220px]"
          />
          <button class="absolute right-3 text-gray-500 hover:text-primary">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" /></svg>
          </button>
        </div>

        <!-- Trái tim (Wishlist) -->
        <button class="text-gray-700 hover:text-primary transition">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
          </svg>
        </button>

        <!-- Giỏ hàng (Cart) -->
        <button class="text-gray-700 hover:text-primary transition relative">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"/>
          </svg>
          <span v-if="isLoggedIn" class="absolute -top-1.5 -right-2 bg-danger text-white text-[10px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
            2
          </span>
        </button>

        <!-- Khối Dropdown Tài khoản -->
        <div 
          v-if="isLoggedIn"
          class="relative ml-2" 
          @mouseenter="isAccountMenuOpen = true" 
          @mouseleave="isAccountMenuOpen = false"
        >
          <button class="w-8 h-8 rounded-full bg-indigo-600 text-white flex items-center justify-center hover:bg-indigo-700 transition cursor-pointer">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
          </button>

          <transition 
            enter-active-class="transition duration-200 ease-out" 
            enter-from-class="transform scale-95 opacity-0" 
            enter-to-class="transform scale-100 opacity-100" 
            leave-active-class="transition duration-150 ease-in" 
            leave-from-class="transform scale-100 opacity-100" 
            leave-to-class="transform scale-95 opacity-0"
          >
            <div 
              v-show="isAccountMenuOpen" 
              class="absolute right-0 top-full mt-3 w-56 bg-[rgba(15,15,15,0.95)] backdrop-blur-md text-gray-200 rounded-lg shadow-2xl py-2 border border-gray-800 z-50 overflow-hidden"
            >
              <a href="#" class="flex items-center gap-3 px-5 py-3 hover:bg-white/10 hover:text-white transition-colors text-sm font-light">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" /></svg>
                Quản lý tài khoản
              </a>
              <a href="#" class="flex items-center gap-3 px-5 py-3 hover:bg-white/10 hover:text-white transition-colors text-sm font-light">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" /></svg>
                Đơn hàng của tôi
              </a>
              <a href="#" class="flex items-center gap-3 px-5 py-3 hover:bg-white/10 hover:text-white transition-colors text-sm font-light">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                Đơn hàng đã hủy
              </a>
              <a href="#" class="flex items-center gap-3 px-5 py-3 hover:bg-white/10 hover:text-white transition-colors text-sm font-light">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" /></svg>
                Yêu thích
              </a>
              
              <!-- Thêm @click.prevent="handleLogout" vào đây -->
              <a href="#" @click.prevent="handleLogout" class="flex items-center gap-3 px-5 py-3 hover:bg-white/10 hover:text-white transition-colors text-sm font-light mt-1 border-t border-white/10">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" /></svg>
                Đăng xuất
              </a>
              
            </div>
          </transition>
        </div>

      </div>
    </div>
  </header>
</template>