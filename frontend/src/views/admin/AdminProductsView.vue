<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'

const products = ref([])
const isLoading = ref(true)

// Hàm gọi API lấy danh sách sản phẩm
const fetchProducts = async () => {
  try {
    const res = await axios.get('http://localhost:3000/api/products')
    // Giả sử backend trả về { data: [...] }
    products.value = res.data.data || res.data
  } catch (error) {
    console.error("Lỗi khi lấy danh sách sản phẩm:", error)
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  fetchProducts()
})

// Hàm format tiền (Chuyển sang $ giống thiết kế hoặc giữ VND tùy bạn)
const formatPrice = (price) => {
  if (!price) return '$0.00'
  return new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(price)
}
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold text-gray-800 mb-8">Sản Phẩm</h2>

    <div v-if="isLoading" class="flex justify-center py-20">
      <div class="animate-spin rounded-full h-10 w-10 border-t-2 border-b-2 border-blue-600"></div>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-3 gap-8">
      
      <div 
        v-for="(product, index) in products.length > 0 ? products : [1,2,3,4,5,6]" 
        :key="product.id || index" 
        class="bg-white rounded-[1.5rem] p-5 shadow-sm border border-gray-50 flex flex-col hover:shadow-md transition-shadow duration-300"
      >
        <div class="relative w-full h-64 bg-[#F8F9FB] rounded-[1rem] mb-5 flex items-center justify-center overflow-hidden group">
          <img 
            :src="product.image_url || 'https://via.placeholder.com/300x300.png?text=Product+Image'" 
            :alt="product.name" 
            class="w-3/4 h-3/4 object-contain"
          />
          
          <button class="absolute left-3 w-8 h-8 bg-white rounded-full flex items-center justify-center shadow-sm text-gray-400 hover:text-gray-800 transition-colors">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/></svg>
          </button>
          
          <button class="absolute right-3 w-8 h-8 bg-white rounded-full flex items-center justify-center shadow-sm text-gray-400 hover:text-gray-800 transition-colors">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/></svg>
          </button>
        </div>

        <div class="flex justify-between items-start mb-1">
          <h3 class="font-bold text-gray-900 text-base truncate pr-4">{{ product.name || 'Apple Watch Series 4' }}</h3>
          <button class="text-gray-300 hover:text-red-500 transition-colors mt-0.5">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
          </button>
        </div>

        <p class="text-blue-500 font-medium text-sm mb-3">{{ formatPrice(product.price || 120) }}</p>

        <div class="flex items-center gap-1 mb-5">
          <div class="flex text-yellow-400">
            <svg v-for="i in 5" :key="i" class="w-4 h-4 fill-current" viewBox="0 0 20 20"><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/></svg>
          </div>
          <span class="text-xs text-gray-400 ml-1">(131)</span>
        </div>

        <div class="mt-auto">
          <button class="bg-[#F5F6FA] text-gray-700 hover:bg-gray-200 hover:text-black px-6 py-2 rounded-lg text-sm font-medium transition-colors">
            Chỉnh sửa
          </button>
        </div>
      </div>

    </div>
  </div>
</template>