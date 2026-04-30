<script setup>
import { ref } from 'vue'
import ProductCard from '@/components/common/ProductCard.vue'

// Mock Data: Sản phẩm trong Wishlist
const wishlistItems = ref([
  { id: 1, name: 'Túi xách thời trang', price: 960000, originalPrice: 1160000, discount: 35, image: '' },
  { id: 2, name: 'Loa RGB Gaming', price: 1960000, originalPrice: null, discount: null, image: '' },
  { id: 3, name: 'Tay cầm DualShock 4', price: 550000, originalPrice: null, discount: null, image: '' },
  { id: 4, name: 'Balo du lịch', price: 750000, originalPrice: null, discount: null, image: '' }
])

// Mock Data: Sản phẩm gợi ý (Just For You)
const suggestedProducts = ref([
  { id: 5, name: 'Laptop ASUS ROG', price: 9600000, originalPrice: 11600000, discount: 35, rating: 5, reviews: 65, image: '' },
  { id: 6, name: 'Màn hình Gaming MSI', price: 1160000, originalPrice: null, discount: null, rating: 4, reviews: 65, image: '' },
  { id: 7, name: 'Tay cầm chơi game Đỏ', price: 560000, originalPrice: null, discount: null, isNew: true, rating: 5, reviews: 65, image: '' },
  { id: 8, name: 'Bàn phím cơ RGB', price: 200000, originalPrice: null, discount: null, rating: 4, reviews: 65, image: '' }
])

// Format tiền tệ VNĐ
const formatPrice = (price) => {
  if (!price) return ''
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price)
}
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20">
    
    <!-- KHU VỰC 1: WISHLIST -->
    <div class="mb-20">
      <div class="flex justify-between items-center mb-8">
        <h2 class="text-xl font-medium text-gray-900">Wishlist ({{ wishlistItems.length }})</h2>
        <button class="border border-gray-300 px-8 py-3 rounded text-sm font-medium hover:bg-gray-50 transition">
          Move All To Bag
        </button>
      </div>

      <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
        <!-- Wishlist Card Custom (Có nút Thùng rác và Thêm vào giỏ hàng cố định) -->
        <div 
          v-for="item in wishlistItems" 
          :key="'wish'+item.id" 
          class="group relative bg-white border border-gray-100 rounded-lg p-4 hover:shadow-xl transition-all duration-300 flex flex-col h-full"
        >
          <div class="relative h-48 bg-[#f5f5f5] rounded-md flex items-center justify-center mb-4 overflow-hidden">
            <span v-if="item.discount" class="absolute top-3 left-3 bg-danger text-white text-xs font-bold px-2 py-1 rounded z-10">
              -{{ item.discount }}%
            </span>
            <!-- Nút xóa (Thùng rác) -->
            <button class="absolute top-3 right-3 bg-white p-1.5 rounded-full shadow hover:bg-gray-100 text-gray-600 transition z-10">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" /></svg>
            </button>
            
            <img v-if="item.image" :src="item.image" :alt="item.name" class="max-h-full object-contain" />
            <span v-else class="text-gray-400 text-sm">Hình ảnh</span>

            <!-- Nút Thêm vào giỏ hàng CỐ ĐỊNH ở thẻ này -->
            <button class="absolute bottom-0 left-0 w-full bg-black text-white py-2 text-sm font-medium hover:bg-gray-800 transition-colors flex items-center justify-center gap-2">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"/></svg>
              Thêm vào giỏ hàng
            </button>
          </div>

          <div class="flex flex-col flex-grow">
            <h3 class="text-sm font-medium text-gray-900 mb-2 line-clamp-2">{{ item.name }}</h3>
            <div class="flex items-center gap-3 mt-auto">
              <span class="text-danger font-bold">{{ formatPrice(item.price) }}</span>
              <span v-if="item.originalPrice" class="text-gray-400 text-xs line-through">{{ formatPrice(item.originalPrice) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- KHU VỰC 2: JUST FOR YOU -->
    <section>
      <div class="flex justify-between items-center mb-8">
        <div class="flex items-center gap-4">
          <div class="w-4 h-8 bg-danger rounded-sm"></div>
          <h2 class="text-xl font-medium text-gray-900">Just For You</h2>
        </div>
        <button class="border border-gray-300 px-8 py-3 rounded text-sm font-medium hover:bg-gray-50 transition">
          See All
        </button>
      </div>

      <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
        <ProductCard v-for="product in suggestedProducts" :key="'suggest'+product.id" :product="product" />
      </div>
    </section>

  </div>
</template>