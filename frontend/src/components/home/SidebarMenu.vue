<script setup>
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

// Đã bổ sung thêm trường "slug" để làm tham số URL
const menuItems = [
  { name: 'TiVi', slug: 'tivi', hasSub: true },
  { name: 'Tủ Lạnh', slug: 'tulanh', hasSub: true },
  { name: 'Máy Tính', slug: 'maytinh', hasSub: false },
  { name: 'Máy Giặt', slug: 'maygiat', hasSub: false },
  { name: 'Loa BlueTooth', slug: 'loa', hasSub: false },
  { name: 'Máy Lọc Nước', slug: 'maylocnuoc', hasSub: false },
  { name: 'Điều hòa', slug: 'dieuhoa', hasSub: false },
  { name: 'Khác', slug: 'khac', hasSub: false },
]

// Hàm xử lý khi người dùng bấm vào một danh mục
const handleCategoryClick = (slug) => {
  // Nếu đang ở mục đó rồi mà bấm lại -> Bỏ lọc, hiện tất cả sản phẩm
  if (route.query.category === slug) {
    router.push({ path: '/products' })
  } else {
    // Chuyển hướng sang trang sản phẩm, gắn thêm category và ép về trang 1
    router.push({ path: '/products', query: { category: slug, page: 1 } })
  }
}
</script>

<template>
  <aside class="border-r border-gray-200 pr-6 pt-10 h-full hidden lg:block">
    <ul class="flex flex-col gap-5 text-sm font-medium">
      <li
          v-for="item in menuItems"
          :key="item.name"
          @click="handleCategoryClick(item.slug)"
          class="flex justify-between items-center cursor-pointer transition-colors group"
          :class="route.query.category === item.slug ? 'text-primary font-bold' : 'text-gray-800 hover:text-primary'"
      >
        <span>{{ item.name }}</span>

        <!-- Icon mũi tên chỉ hiện khi có menu con -->
        <svg
            v-if="item.hasSub"
            class="w-4 h-4 transition-colors"
            :class="route.query.category === item.slug ? 'text-primary' : 'text-gray-500 group-hover:text-primary'"
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