<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { cartCount, fetchCartCount } from '@/store/cartState'
import { wishlistCount, fetchWishlist } from '@/store/wishlistState'

const searchQuery = ref('')
const router = useRouter()
const user = ref(null)

const handleSearch = () => {
  if (searchQuery.value.trim() !== '') {
    router.push({ path: '/products', query: { search: searchQuery.value.trim() } })
  } else {
    // Nếu xóa trắng ô tìm kiếm và enter -> Trở về danh sách gốc
    router.push({ path: '/products' }) 
  }
}

// Quản lý trạng thái mở/đóng của dropdown tài khoản (Hover)
const isAccountMenuOpen = ref(false)

// 1. Khởi tạo trạng thái mặc định là CHƯA ĐĂNG NHẬP
const isLoggedIn = ref(false)
const userName = ref('')

// 2. Hàm onMounted sẽ tự động chạy ngay khi Header được hiển thị
onMounted(() => {
  const storedUser = localStorage.getItem('user')
  // Thêm điều kiện chặn đứng chữ 'undefined'
  if (storedUser && storedUser !== 'undefined') {
    try {
      user.value = JSON.parse(storedUser)
      isLoggedIn.value = true // Đã fix logic cập nhật trạng thái
    } catch (e) {
      console.error("Dữ liệu user bị lỗi, tiến hành xóa:", e)
      localStorage.removeItem('user') 
    }
  }
  fetchCartCount()
  fetchWishlist() // Cập nhật số lượng giỏ hàng ngay khi Header load
})

// 3. Hàm xử lý khi bấm Đăng xuất
const handleLogout = () => {
  localStorage.removeItem('token')
  localStorage.removeItem('user')
  user.value = null
  isLoggedIn.value = false
  isAccountMenuOpen.value = false
  router.push('/login')
}

// 4. Hàm chuyển trang profile
const goToProfile = () => {
  isAccountMenuOpen.value = false // Đóng menu
  router.push('/profile')      // Chuyển trang
}
</script>

<template>
  <header class="bg-white border-b border-gray-100 sticky top-0 z-50">
    <div class="container mx-auto px-4 py-4 flex items-center justify-between max-w-6xl">
      
      <router-link to="/" class="text-2xl font-bold text-gray-900 tracking-tight">
        Cửa Hàng Điện Máy
      </router-link>

      <nav class="hidden md:flex items-center gap-8 text-sm font-medium text-gray-700">
        <router-link to="/" class="hover:text-blue-600 transition">Trang chủ</router-link>
        <router-link to="/products" class="hover:text-blue-600 transition">Sản Phẩm</router-link>
        <router-link to="/contact" class="hover:text-blue-600 transition">Liên hệ</router-link>
        <router-link to="/about" class="hover:text-blue-600 transition">Về chúng tôi</router-link>
        
        <div v-if="!user">
          <router-link to="/login" class="text-sm text-gray-700 hover:text-blue-600 transition">
            Đăng nhập
          </router-link>
        </div>
      </nav>

      <div class="flex items-center gap-5">
        
        <div class="flex items-center border border-gray-300 rounded px-3 py-1.5 bg-gray-50">
          <input 
            v-model="searchQuery"
            @keyup.enter="handleSearch"
            type="text" 
            placeholder="Tìm kiếm điều bạn cần..." 
            class="w-full bg-transparent outline-none text-sm text-gray-700"/>
          
          <button @click="handleSearch" class="text-gray-500 hover:text-blue-600 ml-2 focus:outline-none">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
            </svg>
          </button>
        </div>

        <router-link to="/wishlist" class="text-gray-700 hover:text-red-500 transition relative block">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
          </svg>
          <span v-if="wishlistCount > 0" class="absolute -top-1.5 -right-2 bg-red-500 text-white text-[10px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
            {{ wishlistCount }}
          </span>
        </router-link>

        <router-link to="/cart" class="text-gray-700 hover:text-blue-600 transition relative block">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"/>
          </svg>
          <span v-if="cartCount > 0" class="absolute -top-1.5 -right-2 bg-red-500 text-white text-[10px] font-bold w-4 h-4 rounded-full flex items-center justify-center">
            {{ cartCount }}
          </span>
        </router-link>

        <div 
          v-if="user"
          class="relative ml-1" 
          @mouseenter="isAccountMenuOpen = true" 
          @mouseleave="isAccountMenuOpen = false"
        >
          <button class="w-8 h-8 rounded-full bg-[#5c17e6] text-white flex items-center justify-center hover:bg-[#4b11c2] transition cursor-pointer focus:outline-none shadow-md">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
          </button>

          <transition 
            enter-active-class="transition duration-200 ease-out" 
            enter-from-class="transform scale-95 opacity-0 translate-y-2" 
            enter-to-class="transform scale-100 opacity-100 translate-y-0" 
            leave-active-class="transition duration-150 ease-in" 
            leave-from-class="transform scale-100 opacity-100 translate-y-0" 
            leave-to-class="transform scale-95 opacity-0 translate-y-2"
          >
            <div 
              v-show="isAccountMenuOpen" 
              class="absolute right-0 top-full mt-2 w-56 bg-gradient-to-b from-[#2a2a2a] to-[#1a1a1a] text-gray-300 rounded-md shadow-2xl py-1 z-50 border border-[#333]"
            >
              <a href="/profile" @click.prevent="goToProfile" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/10 hover:text-white transition-colors text-[13px] font-light">
                <svg class="w-4 h-4 opacity-80" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" /></svg>
                Quản lý tài khoản
              </a>
              
              <a href="/orders" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/10 hover:text-white transition-colors text-[13px] font-light">
                <svg class="w-4 h-4 opacity-80" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z" /></svg>
                Đơn hàng của tôi
              </a>
              
              <a href="/cancelled-orders" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/10 hover:text-white transition-colors text-[13px] font-light">
                <svg class="w-4 h-4 opacity-80" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                Đơn hàng đã hủy
              </a>
              
              <a href="/wishlist" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/10 hover:text-white transition-colors text-[13px] font-light">
                <svg class="w-4 h-4 opacity-80" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" /></svg>
                Yêu thích
              </a>
              
              <a href="/logout" @click.prevent="handleLogout" class="flex items-center gap-3 px-4 py-2.5 hover:bg-white/10 hover:text-white transition-colors text-[13px] font-light">
                <svg class="w-4 h-4 opacity-80" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" /></svg>
                Đăng xuất
              </a>
              
            </div>
          </transition>
        </div>

      </div>
    </div>
  </header>
</template>