<script setup>
import { ref } from 'vue'
import SidebarMenu from '@/components/home/SidebarMenu.vue'
import HeroBanner from '@/components/home/HeroBanner.vue'
import CategoryList from '@/components/home/CategoryList.vue'
import ProductCard from '@/components/common/ProductCard.vue'
import ServiceFeatures from '@/components/home/ServiceFeatures.vue'

// Mock Data (Dữ liệu giả lập - Tái sử dụng chung cấu trúc)
const productList = ref([
  { id: 1, name: 'Tay cầm chơi game DualShock 4 Đỏ', image: '', price: 550000, originalPrice: 850000, discount: 35, isNew: false, rating: 4, reviews: 88 },
  { id: 2, name: 'Bàn phím cơ Gaming RGB', image: '', price: 850000, originalPrice: 1200000, discount: 29, isNew: false, rating: 5, reviews: 45 },
  { id: 3, name: 'Màn hình LCD 27 inch 144Hz', image: '', price: 3500000, originalPrice: 4200000, discount: 16, isNew: false, rating: 4, reviews: 120 },
  { id: 4, name: 'Chuột không dây Logitech', image: '', price: 250000, originalPrice: 350000, discount: 28, isNew: false, rating: 5, reviews: 300 },
  { id: 5, name: 'Laptop Dell XPS 15 Cảm ứng', image: '', price: 35000000, originalPrice: 35000000, discount: 0, isNew: true, rating: 5, reviews: 24 }
])
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl mb-12">
    
    <!-- KHU VỰC 1: Cấu trúc chia đôi Sidebar (Menu) và Banner -->
    <div class="flex flex-col lg:flex-row gap-10 mb-20">
      <!-- Cột trái: Sidebar Menu (Chiếm khoảng 1/4) -->
      <div class="lg:w-1/4">
        <SidebarMenu />
      </div>
      <!-- Cột phải: Banner (Chiếm khoảng 3/4) -->
      <div class="lg:w-3/4 pt-10">
        <HeroBanner />
      </div>
    </div>

    <!-- KHU VỰC 2: Khối Flash Sales -->
    <section class="mb-20">
      <div class="flex items-center gap-4 mb-8">
        <div class="w-4 h-8 bg-danger rounded-sm"></div>
        <h2 class="text-xl font-bold text-danger">Flash Sales</h2>
      </div>
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-6">
        <ProductCard v-for="product in productList" :key="'flash'+product.id" :product="product" />
      </div>
      <div class="flex justify-center mt-10">
        <button class="bg-danger text-white px-10 py-3 rounded text-sm font-medium hover:bg-red-600 transition">Xem Tất Cả Sản Phẩm</button>
      </div>
    </section>

    <!-- KHU VỰC 3: Khối Danh mục -->
    <CategoryList class="mb-20" />

    <!-- KHU VỰC 4: Sản Phẩm Bán Chạy -->
    <section class="mb-20">
      <div class="flex items-center gap-4 mb-8">
        <div class="w-4 h-8 bg-danger rounded-sm"></div>
        <h2 class="text-xl font-bold text-danger">Sản Phẩm Bán Chạy</h2>
      </div>
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-6">
        <!-- Render ngược mảng để thấy sự khác biệt của data -->
        <ProductCard v-for="product in productList.slice().reverse()" :key="'best'+product.id" :product="product" />
      </div>
    </section>

    <!-- KHU VỰC 5: Banner ngang (Loa Bluetooth) -->
    <section class="mb-20">
      <div class="bg-black text-white rounded-lg p-10 flex flex-col md:flex-row items-center justify-between">
        <div class="space-y-6 md:w-1/2 z-10">
          <span class="text-green-500 text-sm font-medium">Trải nghiệm âm nhạc</span>
          <h2 class="text-4xl md:text-5xl font-bold leading-tight">Loa Bluetooth<br>Bùng Nổ</h2>
          <button class="bg-green-500 text-white px-8 py-3 rounded text-sm font-medium hover:bg-green-600 transition">Mua Ngay</button>
        </div>
        <div class="md:w-1/2 flex justify-end mt-8 md:mt-0 z-0 opacity-50 md:opacity-100">
          <div class="w-64 h-48 bg-gray-800 rounded border border-gray-700 flex items-center justify-center text-gray-400">Hình ảnh Loa</div>
        </div>
      </div>
    </section>

    <!-- KHU VỰC 6: Sản Phẩm Nổi Bật -->
    <section class="mb-20">
      <div class="flex items-center gap-4 mb-8">
        <div class="w-4 h-8 bg-primary rounded-sm"></div>
        <h2 class="text-xl font-bold text-primary">Sản Phẩm Nổi Bật</h2>
      </div>
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-6">
        <ProductCard v-for="product in productList" :key="'feat'+product.id" :product="product" />
      </div>
      <div class="flex justify-center mt-10">
        <button class="bg-primary text-white px-10 py-3 rounded text-sm font-medium hover:bg-blue-600 transition">Xem Thêm Sản Phẩm</button>
      </div>
    </section>

    <!-- KHU VỰC 7: Sản Phẩm Mới -->
    <section class="mb-24">
      <div class="flex items-center gap-4 mb-8">
        <div class="w-4 h-8 bg-primary rounded-sm"></div>
        <h2 class="text-xl font-bold text-primary">Sản Phẩm Mới</h2>
      </div>
      <!-- Thiết kế dạng Grid đặc biệt cho Sản phẩm mới (dựa theo Figma thường có khối lớn khối nhỏ) -->
      <!-- Tạm thời sử dụng grid tĩnh chia ô lớn/nhỏ -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 h-[600px]">
        <!-- Box lớn bên trái (Ví dụ PS5) -->
        <div class="bg-black text-white rounded-lg p-8 flex flex-col justify-end relative overflow-hidden group cursor-pointer">
          <div class="absolute inset-0 bg-gray-900 flex items-center justify-center text-gray-600">Hình PS5</div>
          <div class="relative z-10">
            <h3 class="text-2xl font-bold mb-2">PlayStation 5</h3>
            <p class="text-sm text-gray-300 mb-4">Trải nghiệm chơi game thế hệ mới.</p>
            <a href="#" class="underline hover:text-gray-300 transition">Mua Ngay</a>
          </div>
        </div>
        <!-- 2 Box nhỏ bên phải -->
        <div class="grid grid-rows-2 gap-6">
          <div class="bg-black text-white rounded-lg p-8 flex flex-col justify-end relative overflow-hidden group cursor-pointer">
             <div class="absolute inset-0 bg-gray-900 flex items-center justify-center text-gray-600">Hình Loa</div>
             <div class="relative z-10">
              <h3 class="text-xl font-bold mb-1">Loa Thông Minh</h3>
              <p class="text-sm text-gray-300 mb-3">Âm thanh sống động.</p>
              <a href="#" class="underline hover:text-gray-300 transition">Mua Ngay</a>
            </div>
          </div>
          <div class="bg-black text-white rounded-lg p-8 flex flex-col justify-end relative overflow-hidden group cursor-pointer">
             <div class="absolute inset-0 bg-gray-900 flex items-center justify-center text-gray-600">Hình Nước Hoa</div>
             <div class="relative z-10">
              <h3 class="text-xl font-bold mb-1">Bộ Sưu Tập Premium</h3>
              <p class="text-sm text-gray-300 mb-3">Đẳng cấp khác biệt.</p>
              <a href="#" class="underline hover:text-gray-300 transition">Mua Ngay</a>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- KHU VỰC 8: Feature Icons (Dịch vụ khách hàng) -->
    <ServiceFeatures />

  </div>
</template>