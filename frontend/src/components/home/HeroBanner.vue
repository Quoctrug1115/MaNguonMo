<template>
  <div class="bg-black text-white rounded-xl overflow-hidden relative mt-8">
    <div class="flex flex-col md:flex-row items-center p-8 md:p-12 lg:p-16">
      
      <!-- Cột Trái: Nội dung chữ -->
      <div class="md:w-1/2 z-10 space-y-6">
        <div class="flex items-center gap-3">
          <!-- Icon Apple/Samsung nhỏ (Dùng SVG tạm) -->
          <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 24 24">
             <path d="M12 2C6.477 2 2 6.477 2 12c0 4.991 3.657 9.128 8.438 9.878v-6.987h-2.54V12h2.54V9.797c0-2.506 1.492-3.89 3.777-3.89 1.094 0 2.238.195 2.238.195v2.46h-1.26c-1.243 0-1.63.771-1.63 1.562V12h2.773l-.443 2.89h-2.33v6.988C18.343 21.128 22 16.991 22 12c0-5.523-4.477-10-10-10z"/>
          </svg>
          <span class="text-sm font-medium">Samsung Series</span>
        </div>
        
        <h2 class="text-4xl md:text-5xl font-bold leading-tight">
          Lên tới 10% <br /> Giảm giá
        </h2>
        
        <div class="pt-4">
          <a href="#" class="inline-flex items-center gap-2 border-b border-white pb-1 hover:text-gray-300 transition">
            Mua Ngay
            <span>→</span>
          </a>
        </div>
      </div>

      <!-- Cột Phải: Hình ảnh sản phẩm -->
      <div class="md:w-1/2 mt-8 md:mt-0 flex justify-center z-10">
        <div class="w-64 h-64 bg-gray-800 rounded flex items-center justify-center text-gray-500 border border-gray-700 shadow-lg">
          [Hình ảnh Samsung QLED 8K]
        </div>
      </div>
      
    </div>
  </div>
</template>