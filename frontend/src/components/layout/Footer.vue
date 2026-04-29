<template>
  <!-- Giảm pt-16 xuống pt-12 để toàn bộ nội dung dịch lên trên một chút -->
  <footer class="bg-primary text-white pt-12 pb-8 mt-auto">
    <div class="container mx-auto px-4 max-w-6xl">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 mb-12">
        
        <!-- Cột 1: Đăng ký nhận tin -->
        <div>
          <h3 class="text-xl font-bold mb-4">Cửa Hàng Điện Máy</h3>
          <p class="text-sm mb-4 font-light text-blue-100">Giảm 10% cho đơn hàng đầu tiên</p>
          <div class="relative w-full max-w-xs">
            <input 
              type="email" 
              placeholder="Nhập email của bạn" 
              class="bg-transparent border border-white/40 rounded px-4 py-2.5 text-sm w-full focus:outline-none focus:border-white placeholder-white/60"
            />
            <button class="absolute right-3 top-3">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
              </svg>
            </button>
          </div>
        </div>

        <!-- Cột 2: Hỗ trợ -->
        <div>
          <h3 class="text-lg font-bold mb-4">Hỗ trợ</h3>
          <ul class="text-sm space-y-3 font-light text-blue-100">
            <li>686 Lê Trọng Tấn, Tân Phú,<br>TP.Hồ Chí Minh</li>
            <li>cuahangdienmay@gmail.com</li>
            <li>+88015-88888-9999</li>
          </ul>
        </div>

        <!-- Cột 3: Quick Link -->
        <div>
          <h3 class="text-lg font-bold mb-4">Quick Link</h3>
          <ul class="text-sm space-y-3 font-light text-blue-100">
            <li><a href="#" class="hover:text-white transition">Privacy Policy</a></li>
            <li><a href="#" class="hover:text-white transition">Terms Of Use</a></li>
            <li><a href="#" class="hover:text-white transition">FAQ</a></li>
            <li><a href="#" class="hover:text-white transition">Contact</a></li>
          </ul>
        </div>

        <!-- Cột 4: Socials (Đã sửa lại để hiện chữ và cân bằng) -->
        <div>
          <h3 class="text-lg font-bold mb-4">Socials</h3>
          <div class="flex gap-5 items-center font-light text-blue-100 text-lg">
            <a href="#" class="hover:text-white transition">
              <span class="font-bold">f</span>
            </a>
            <a href="#" class="hover:text-white transition">
              <span class="font-bold">tw</span>
            </a>
            <a href="#" class="hover:text-white transition">
              <span class="font-bold">ig</span>
            </a>
            <a href="#" class="hover:text-white transition">
              <span class="font-bold">in</span>
            </a>
          </div>
        </div>

      </div>
    </div>
  </footer>
</template>

<script setup>
</script>