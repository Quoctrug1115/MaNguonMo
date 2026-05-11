<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import axios from 'axios'

const products = ref([])
const isLoading = ref(true)

// --- STATE TÌM KIẾM & PHÂN TRANG ---
const searchQuery = ref('') // Biến lưu từ khóa tìm kiếm
const currentPage = ref(1)
const itemsPerPage = 9

const fetchProducts = async () => {
  try {
    isLoading.value = true
    const res = await axios.get('http://localhost:3000/api/admin/product-variants')
    
    products.value = (res.data.data || res.data).map(p => ({
      ...p,
      display_image: p.main_image
    }))
  } catch (error) {
    console.error("Lỗi:", error)
  } finally {
    isLoading.value = false
  }
}

onMounted(() => { fetchProducts() })

const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price)
}

const translateCategory = (categoryName) => {
  const dictionary = {
    'Digital Product': 'Thiết Bị Số',
    'Fashion': 'Thời Trang',
    'Mobile': 'Điện Thoại',
    'Electronic': 'Điện Máy',
    'Gaming': 'Máy Tính & Game',
  }
  return dictionary[categoryName] || categoryName || 'Chưa phân loại'
}

// --- LOGIC TÌM KIẾM ---
const filteredProducts = computed(() => {
  if (!searchQuery.value.trim()) return products.value
  
  const lowerCaseQuery = searchQuery.value.toLowerCase().trim()
  
  return products.value.filter(p => {
    const nameMatch = p.product_name.toLowerCase().includes(lowerCaseQuery)
    const categoryMatch = translateCategory(p.category).toLowerCase().includes(lowerCaseQuery)
    return nameMatch || categoryMatch
  })
})

watch(searchQuery, () => {
  currentPage.value = 1
})

// --- LOGIC PHÂN TRANG ---
const paginatedProducts = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  const end = start + itemsPerPage
  return filteredProducts.value.slice(start, end)
})

const totalPages = computed(() => {
  return Math.ceil(filteredProducts.value.length / itemsPerPage)
})

const nextPage = () => {
  if (currentPage.value < totalPages.value) currentPage.value++
}
const prevPage = () => {
  if (currentPage.value > 1) currentPage.value--
}
</script>

<template>
  <div class="p-8 max-w-7xl mx-auto">
    <div class="flex justify-between items-center mb-8">
      <h2 class="text-[28px] font-bold text-gray-800 tracking-wide">Quản Lý Sản Phẩm</h2>
      
      <div class="flex gap-4 items-center">
        <div class="flex items-center gap-3 bg-white border border-gray-100 rounded-full w-[300px] h-[46px] px-4 shadow-sm focus-within:border-blue-500 focus-within:ring-2 focus-within:ring-blue-100 transition-all">
          <svg class="w-4 h-4 text-gray-400 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
          </svg>
          
          <input 
            v-model="searchQuery" 
            type="text" 
            placeholder="Tìm kiếm tên sản phẩm..." 
            class="flex-grow bg-transparent text-sm text-gray-700 placeholder:text-gray-400 outline-none h-full" 
          />
        </div>
        <button class="bg-blue-600 text-white px-5 py-2.5 rounded-full text-sm font-bold shadow-md hover:bg-blue-700 transition-colors h-[46px]">
          + Thêm Sản Phẩm Mới
        </button>
      </div>
    </div>

    <div class="bg-white rounded-2xl shadow-[0_2px_10px_rgba(0,0,0,0.04)] border border-gray-100 overflow-hidden">
      
      <div v-if="isLoading" class="flex justify-center py-20">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
      
      <div v-else-if="filteredProducts.length === 0" class="flex flex-col items-center justify-center py-20 text-gray-500">
        <svg class="w-16 h-16 mb-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
        <p class="text-lg font-medium">Không tìm thấy sản phẩm nào</p>
        <p class="text-sm">Hãy thử tìm với từ khóa khác (Ví dụ: Apple, Tivi...)</p>
      </div>

      <table v-else class="w-full text-left border-collapse">
        <thead>
          <tr class="text-gray-800 text-sm font-bold border-b border-gray-100 bg-gray-50/50">
            <th class="px-6 py-5 w-24 text-center">Hình Ảnh</th>
            <th class="px-6 py-5">Tên Sản Phẩm</th>
            <th class="px-6 py-5">Danh Mục</th>
            <th class="px-6 py-5">Giá Bán</th>
            <th class="px-6 py-5">Tổng Tồn</th>
            <th class="px-6 py-5">Tồn Kho (Màu)</th>
            <th class="px-6 py-5">Màu Sắc</th>
            <th class="px-6 py-5 text-center">Thao Tác</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-100 text-sm font-medium text-gray-600">
          
          <tr v-for="product in paginatedProducts" :key="product.id" class="hover:bg-gray-50 transition-colors">
            
            <td class="px-6 py-4 flex justify-center">
              <img :src="product.display_image || 'https://via.placeholder.com/150'" class="w-14 h-14 rounded-xl object-cover border border-gray-100 shadow-sm transition-all duration-300 bg-white" />
            </td>
            
            <td class="px-6 py-4 text-gray-800">{{ product.product_name }}</td>
            <td class="px-6 py-4">{{ translateCategory(product.category) }}</td>
            <td class="px-6 py-4 font-bold text-gray-700">{{ formatPrice(product.price) }}</td>
            <td class="px-6 py-4">
              <span class="px-3 py-1 bg-gray-100 text-gray-700 rounded-md font-bold">{{ product.total_stock }}</span>
            </td>
            
            <td class="px-6 py-4">
              <span v-if="product.variants.length === 0" class="text-xs text-gray-400 italic">Chưa có dữ liệu</span>
              <div v-else class="flex flex-wrap gap-2">
                <div 
                  v-for="variant in product.variants" 
                  :key="'stock-' + variant.variant_id" 
                  class="flex items-center gap-1.5 bg-white border border-gray-200 px-2 py-1 rounded shadow-sm"
                >
                  <span 
                    :style="{ 'background-color': variant.color_hex ? variant.color_hex.trim() : '#cccccc' }" 
                    class="w-2.5 h-2.5 block rounded-full border border-gray-300"
                  ></span>
                  <span :class="variant.stock < 10 ? 'text-red-500' : 'text-gray-700'" class="text-xs font-bold">
                    {{ variant.stock }}
                  </span>
                </div>
              </div>
            </td>

            <td class="px-6 py-4">
              <div class="flex gap-2.5 items-center">
                <span v-if="product.variants.length === 0" class="text-xs text-gray-400 italic">-</span>
                <span 
                  v-else 
                  v-for="variant in product.variants" 
                  :key="variant.variant_id" 
                  :style="{ 'background-color': variant.color_hex ? variant.color_hex.trim() : '#cccccc' }" 
                  @mouseover="product.display_image = variant.image_url || product.main_image"
                  @mouseleave="product.display_image = product.main_image"
                  class="w-5 h-5 block rounded-full shadow-sm cursor-pointer hover:ring-2 hover:ring-blue-500 hover:shadow-lg transition-all border border-gray-200"
                  :title="'Mã màu: ' + (variant.color_hex || 'Trống')"
                ></span>
              </div>
            </td>
            
            <td class="px-6 py-4">
              <div class="flex justify-center gap-2">
                <button title="Chỉnh sửa" class="p-2.5 border border-gray-200 text-gray-400 hover:text-blue-600 hover:border-blue-200 rounded-lg transition-colors bg-white shadow-sm">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"/></svg>
                </button>
                <button title="Xóa" class="p-2.5 border border-gray-200 text-gray-400 hover:text-red-600 hover:border-red-200 rounded-lg transition-colors bg-white shadow-sm">
                  <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>
                </button>
              </div>
            </td>
          </tr>
          
        </tbody>
      </table>
      
      <div v-if="!isLoading && filteredProducts.length > 0" class="px-6 py-4 flex justify-between items-center border-t border-gray-100 bg-white">
        <p class="text-sm font-medium text-gray-500">
          Hiển thị <span class="text-gray-800 font-bold">{{ (currentPage - 1) * itemsPerPage + 1 }}</span> 
          - <span class="text-gray-800 font-bold">{{ Math.min(currentPage * itemsPerPage, filteredProducts.length) }}</span> 
          trong tổng số <span class="text-gray-800 font-bold">{{ filteredProducts.length }}</span> kết quả
        </p>
        <div class="flex gap-2">
          <button 
            @click="prevPage" 
            :disabled="currentPage === 1"
            :class="currentPage === 1 ? 'opacity-50 cursor-not-allowed bg-gray-50' : 'hover:bg-blue-50 hover:text-blue-600 hover:border-blue-200'"
            class="px-3 py-1.5 rounded-lg border border-gray-200 text-gray-500 font-bold transition-colors shadow-sm bg-white"
          >
            &lt;
          </button>
          <button 
            @click="nextPage" 
            :disabled="currentPage === totalPages || totalPages === 0"
            :class="currentPage === totalPages || totalPages === 0 ? 'opacity-50 cursor-not-allowed bg-gray-50' : 'hover:bg-blue-50 hover:text-blue-600 hover:border-blue-200'"
            class="px-3 py-1.5 rounded-lg border border-gray-200 text-gray-500 font-bold transition-colors shadow-sm bg-white"
          >
            &gt;
          </button>
        </div>
      </div>
      
    </div>
  </div>
</template>