<template>
  <div class="min-h-screen flex flex-col bg-gray-50">
    <!-- Header Area -->
    <header class="bg-white shadow-sm sticky top-0 z-50">
      <!-- TopBar: Dải nhỏ trên cùng -->
      <div class="container mx-auto px-4 py-1 text-xs text-gray-500 flex justify-end gap-4">
        <span>Trợ giúp</span>
        <span>Tiếng Việt</span>
      </div>
      
      <!-- Main Header -->
      <div class="container mx-auto px-4 py-4 flex items-center justify-between">
        <!-- Logo -->
        <h1 class="text-2xl font-bold text-gray-800">Cửa Hàng Điện Máy</h1>
        
        <!-- Search Bar -->
        <div class="flex-1 max-w-2xl mx-8 relative">
          <input 
            type="text" 
            placeholder="Tìm kiếm sản phẩm..." 
            class="w-full px-4 py-2 border rounded-full focus:outline-none focus:border-primary"
          />
          <button class="absolute right-3 top-2.5 text-gray-400">
            <!-- Icon Search (dùng Heroicons hoặc SVG tĩnh) -->
            🔍
          </button>
        </div>

        <!-- Actions (Cart, Account) -->
        <div class="flex items-center gap-6">
          <button class="flex flex-col items-center hover:text-primary">
            <span>🛒</span>
            <span class="text-xs">Giỏ hàng</span>
          </button>
          <button class="flex flex-col items-center hover:text-primary">
            <span>👤</span>
            <span class="text-xs">Tài khoản</span>
          </button>
        </div>
      </div>
    </header>

    <!-- Main Content Area (Nội dung thay đổi theo Route) -->
    <main class="flex-grow container mx-auto px-4 py-6">
      <slot></slot>
      <!-- Sau này khi ráp Vue Router, thẻ <slot> sẽ được thay bằng <router-view /> -->
    </main>

    <!-- Footer Area -->
    <footer class="bg-primary text-white pt-12 pb-6 mt-12">
      <div class="container mx-auto px-4 grid grid-cols-1 md:grid-cols-4 gap-8">
        <div>
          <h2 class="text-xl font-bold mb-4">Cửa Hàng Điện Máy</h2>
          <p class="text-sm">Giảm 10% cho đơn hàng đầu tiên</p>
          <!-- Email Input -->
        </div>
        <div>
          <h3 class="font-bold mb-4">Hỗ trợ</h3>
          <p class="text-sm">686 Lê Trọng Tấn, Tân Phú, TP.HCM</p>
          <p class="text-sm mt-2">cuahangdienmay@gmail.com</p>
        </div>
        <div>
          <h3 class="font-bold mb-4">Nhấn liên kết</h3>
          <ul class="text-sm space-y-2">
            <li>Privacy Policy</li>
            <li>Terms Of Use</li>
            <li>FAQ</li>
          </ul>
        </div>
        <div>
           <!-- Social Icons -->
        </div>
      </div>
    </footer>
  </div>
</template>