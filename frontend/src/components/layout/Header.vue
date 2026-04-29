<template>
  <header class="bg-white border-b border-gray-100 sticky top-0 z-50">
    <div class="container mx-auto px-4 py-5 flex items-center justify-between max-w-6xl">
      
      <!-- Logo -->
      <router-link to="/" class="text-2xl font-bold text-gray-900 tracking-tight">
        Cửa Hàng Điện Máy
      </router-link>

      <!-- Navigation Links -->
      <nav class="hidden md:flex items-center gap-8 text-sm font-medium text-gray-700">
        <router-link to="/" class="hover:text-primary transition">Trang chủ</router-link>
        <a href="#" class="hover:text-primary transition">Liên hệ</a>
        <a href="#" class="hover:text-primary transition">Về chúng tôi</a>
        <router-link to="/login" class="hover:text-primary transition">Đăng nhập</router-link>
      </nav>

      <!-- Search Bar -->
      <div class="hidden lg:flex items-center relative">
        <input 
          type="text" 
          placeholder="Tìm kiếm điều bạn cần" 
          class="bg-[#f5f5f5] text-sm rounded-md pl-4 pr-10 py-2.5 focus:outline-none focus:ring-1 focus:ring-primary w-[250px]"
        />
        <button class="absolute right-3 text-gray-500">
          <!-- Icon Search SVG -->
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </button>
      </div>

    </div>
  </header>
</template>

<script setup>
// Header logic (nếu có sau này, ví dụ: xử lý search)
</script>