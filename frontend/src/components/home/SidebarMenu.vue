<script setup>
const menuItems = [
  { name: 'TiVi', hasSub: true },
  { name: 'Tủ Lạnh', hasSub: true },
  { name: 'Máy Tính', hasSub: false },
  { name: 'Máy Giặt', hasSub: false },
  { name: 'Loa BlueTooth', hasSub: false },
  { name: 'Máy Lọc Nước', hasSub: false },
  { name: 'Điện Thoại', hasSub: false },
  { name: 'Đồng Hồ', hasSub: false },
  { name: 'Khác', hasSub: false },
]
</script>

<template>
  <aside class="border-r border-gray-200 pr-6 pt-10 h-full hidden lg:block">
    <ul class="flex flex-col gap-5 text-sm font-medium text-gray-800">
      <li 
        v-for="item in menuItems" 
        :key="item.name" 
        class="flex justify-between items-center cursor-pointer hover:text-primary transition-colors group"
      >
        <span>{{ item.name }}</span>
        <!-- Icon mũi tên chỉ hiện khi có menu con -->
        <svg 
          v-if="item.hasSub" 
          class="w-4 h-4 text-gray-500 group-hover:text-primary" 
          fill="none" 
          stroke="currentColor" 
          viewBox="0 0 24 24"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
        </svg>
      </li>
    </ul>
  </aside>
</template>