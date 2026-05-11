<script setup>
// Nhúng Header chuẩn mà chúng ta đã làm (có Dropdown, giỏ hàng, Wishlist...)
import Header from '@/components/Header.vue' 
</script>

<template>
  <div class="min-h-screen flex flex-col bg-gray-50">
    
    <Header />

    <main class="flex-grow">
      <router-view />
    </main>

    <footer class="bg-blue-600 text-white pt-12 pb-6 mt-12">
      <div class="container mx-auto px-4 grid grid-cols-1 md:grid-cols-4 gap-8 max-w-6xl">
        <div>
          <h2 class="text-xl font-bold mb-4">Cửa Hàng Điện Máy</h2>
          <p class="text-sm">Giảm 10% cho đơn hàng đầu tiên</p>
          </div>
        <div>
          <h3 class="font-bold mb-4">Hỗ trợ</h3>
          <p class="text-sm">686 Lê Trọng Tấn, Tân Phú, TP.HCM</p>
          <p class="text-sm mt-2">cuahangdienmay@gmail.com</p>
        </div>
        <div>
          <h3 class="font-bold mb-4">Nhấn liên kết</h3>
          <ul class="text-sm space-y-2 cursor-pointer">
            <li class="hover:underline">Privacy Policy</li>
            <li class="hover:underline">Terms Of Use</li>
            <li class="hover:underline">FAQ</li>
          </ul>
        </div>
        <div>
           </div>
      </div>
    </footer>
    
  </div>
</template>