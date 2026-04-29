<script setup>
// Dữ liệu mô phỏng danh mục (sử dụng SVG path cho các icon)
const categories = [
  { id: 1, name: 'Điện Thoại', icon: 'M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z' },
  { id: 2, name: 'Máy Tính', icon: 'M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z' },
  { id: 3, name: 'Đồng Hồ', icon: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z' },
  { id: 4, name: 'Máy Ảnh', icon: 'M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z M15 13a3 3 0 11-6 0 3 3 0 016 0z' },
  { id: 5, name: 'Tai Nghe', icon: 'M21 15a2 2 0 01-2 2H7l-4 4V5a2 2 0 012-2h14a2 2 0 012 2v10z' }, 
  { id: 6, name: 'Gaming', icon: 'M15 11a3 3 0 11-6 0 3 3 0 016 0z' }
]
</script>

<template>
  <section class="mt-20">
    <!-- Tiêu đề phụ (Chữ đỏ có vạch) -->
    <div class="flex items-center gap-4 mb-4">
      <div class="w-4 h-8 bg-danger rounded-sm"></div>
      <h2 class="text-sm font-bold text-danger uppercase tracking-wider">Danh mục</h2>
    </div>
    
    <!-- Tiêu đề chính & Nút điều hướng -->
    <div class="flex items-center justify-between mb-10">
      <h3 class="text-3xl font-bold text-gray-900 tracking-tight">Khám Phá Theo Danh Mục</h3>
      <div class="flex gap-2">
        <button class="w-11 h-11 rounded-full bg-gray-100 flex items-center justify-center hover:bg-gray-200 transition">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/></svg>
        </button>
        <button class="w-11 h-11 rounded-full bg-gray-100 flex items-center justify-center hover:bg-gray-200 transition">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"/></svg>
        </button>
      </div>
    </div>

    <!-- Grid danh mục -->
    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-6">
      <div 
        v-for="cat in categories" 
        :key="cat.id" 
        class="border border-gray-200 rounded-md py-8 px-4 flex flex-col items-center justify-center gap-4 cursor-pointer group hover:bg-primary hover:border-primary transition-all duration-300 shadow-sm hover:shadow-md bg-white"
      >
        <svg class="w-12 h-12 text-gray-900 group-hover:text-white transition-colors duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.2" :d="cat.icon" />
        </svg>
        <span class="text-sm font-medium group-hover:text-white transition-colors duration-300">{{ cat.name }}</span>
      </div>
    </div>
  </section>
</template>