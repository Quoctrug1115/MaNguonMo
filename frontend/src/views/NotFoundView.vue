<script setup>
// Trang này chỉ thuần hiển thị tĩnh nên không cần logic JS phức tạp
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20">
    
    <!-- Breadcrumb -->
    <div class="text-sm text-gray-500 mb-20">
      <router-link to="/" class="hover:text-primary transition">Home</router-link> / 
      <span class="text-gray-900 font-medium">404 Error</span>
    </div>

    <!-- Main Content 404 -->
    <div class="flex flex-col items-center justify-center text-center pb-24">
      
      <!-- Chữ 404 siêu lớn -->
      <h1 class="text-6xl md:text-8xl font-medium text-gray-900 mb-8 tracking-widest">
        404 Not Found
      </h1>
      
      <!-- Lời nhắn -->
      <p class="text-gray-600 mb-16">
        Your visited page not found. You may go home page.
      </p>
      
      <!-- Nút Về Trang Chủ -->
      <router-link 
        to="/" 
        class="bg-primary text-white px-12 py-4 rounded text-sm font-medium hover:bg-blue-600 transition shadow-sm"
      >
        Về Trang Chủ
      </router-link>

    </div>

  </div>
</template>