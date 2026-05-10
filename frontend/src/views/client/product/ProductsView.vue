<script setup>
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ProductCard from '@/components/common/ProductCard.vue'
import SidebarMenu from '@/components/home/SidebarMenu.vue'
import axios from 'axios'

const route = useRoute()
const router = useRouter()

const productList = ref([])
const isLoading = ref(true)
const currentCategory = ref(route.query.category || '')

const minPrice = ref(route.query.min_price || '')
const maxPrice = ref(route.query.max_price || '')

const currentPage = ref(1)
const totalPages = ref(1)

// Hàm gọi API lấy sản phẩm
const fetchProducts = async () => {
  isLoading.value = true
  try {
    const params = {}
    
    if (route.query.search) params.search = route.query.search
    if (route.query.min_price) params.min_price = route.query.min_price
    if (route.query.max_price) params.max_price = route.query.max_price
    if (route.query.category) params.category_id = route.query.category 

    const res = await axios.get('http://localhost:3000/api/products', { params })
    productList.value = res.data.data
    
  } catch (error) {
    console.error("Lỗi tải sản phẩm:", error)
  } finally {
    isLoading.value = false
  }
}

// Chuyển trang
const goToPage = (page) => {
  if (page < 1 || page > totalPages.value || page === currentPage.value) return
  router.push({ path: '/products', query: { ...route.query, page: page } })
}

// ----------------------------------------------------
// CÁC HÀM XỬ LÝ LỌC GIÁ TIỀN (VỪA THÊM VÀO)
// ----------------------------------------------------
const applyFilters = () => {
  const currentQuery = { ...route.query }
  
  if (minPrice.value) currentQuery.min_price = minPrice.value
  else delete currentQuery.min_price

  if (maxPrice.value) currentQuery.max_price = maxPrice.value
  else delete currentQuery.max_price

  // Khi lọc giá mới thì nên reset về trang 1
  currentQuery.page = 1 

  router.push({ query: currentQuery })
}

const clearFilters = () => {
  minPrice.value = ''
  maxPrice.value = ''
  const currentQuery = { ...route.query }
  delete currentQuery.min_price
  delete currentQuery.max_price
  currentQuery.page = 1
  router.push({ query: currentQuery })
}

// ----------------------------------------------------
// WATCH DUY NHẤT: Bắt mọi sự thay đổi trên URL
// ----------------------------------------------------
watch(() => route.query, (newQuery) => {
  currentPage.value = parseInt(newQuery.page) || 1
  currentCategory.value = newQuery.category || ''
  minPrice.value = newQuery.min_price || ''
  maxPrice.value = newQuery.max_price || ''
  
  fetchProducts()
}, { deep: true })

onMounted(() => {
  currentPage.value = parseInt(route.query.page) || 1
  fetchProducts()
})
</script>

<template>
  <div class="container mx-auto px-4 max-w-7xl py-10 mb-20">

    <div class="mb-8 border-b pb-4 flex items-center justify-between">
      <h1 class="text-3xl font-bold text-gray-900">
        {{ route.query.search ? `Kết quả cho: "${route.query.search}"` : 'Tất Cả Sản Phẩm' }}
      </h1>
    </div>

    <div class="flex flex-col lg:flex-row gap-8 items-start">

      <div class="w-full lg:w-1/4 flex-shrink-0 min-w-[250px]">
        
        <div class="sticky top-20 flex flex-col gap-6">
          
          <div class="bg-white rounded-lg border border-gray-200 shadow-sm p-4 hidden lg:block">
            <SidebarMenu />
          </div>

          <div class="bg-white p-5 rounded-lg border border-gray-200 shadow-sm">
            <h2 class="font-bold text-lg border-b pb-3 mb-4">Lọc Theo Giá</h2>
            
            <div class="mb-6">
              <h3 class="font-medium text-gray-700 mb-3">Khoảng giá (VNĐ)</h3>
              <div class="flex items-center gap-2 mb-3">
                <input v-model="minPrice" type="number" placeholder="Từ..." class="w-full border border-gray-300 rounded px-2 py-1.5 text-sm outline-none focus:border-blue-500" />
                <span class="text-gray-500">-</span>
                <input v-model="maxPrice" type="number" placeholder="Đến..." class="w-full border border-gray-300 rounded px-2 py-1.5 text-sm outline-none focus:border-blue-500" />
              </div>
            </div>

            <div class="flex flex-col gap-2">
              <button @click="applyFilters" class="bg-blue-600 text-white w-full py-2 rounded hover:bg-blue-700 transition font-medium text-sm">
                Áp dụng
              </button>
              <button @click="clearFilters" class="bg-gray-100 text-gray-600 w-full py-2 rounded hover:bg-gray-200 transition font-medium text-sm">
                Xóa bộ lọc
              </button>
            </div>
          </div>

        </div>
      </div>

      <div class="w-full flex-1 overflow-hidden">

        <div v-if="isLoading" class="flex justify-center items-center py-32">
          <div class="animate-spin rounded-full h-10 w-10 border-t-2 border-b-2 border-blue-600"></div>
        </div>

        <div v-else>
          <div v-if="productList.length === 0" class="text-center py-20 border border-gray-100 bg-gray-50 rounded-lg">
            <p class="text-gray-500 text-lg">Không tìm thấy sản phẩm nào phù hợp.</p>
          </div>

          <div v-else class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-6">
            <ProductCard v-for="product in productList" :key="product.id" :product="product" />
          </div>

          <div v-if="totalPages > 1" class="flex justify-center items-center gap-2 mt-12">
            <button @click="goToPage(currentPage - 1)" :disabled="currentPage === 1" class="px-4 py-2 border rounded-md" :class="currentPage === 1 ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'hover:bg-gray-50 text-blue-600'">&laquo; Trước</button>
            <div class="hidden sm:flex gap-1">
              <button v-for="page in totalPages" :key="page" @click="goToPage(page)" class="w-10 h-10 border rounded-md flex items-center justify-center font-medium" :class="page === currentPage ? 'bg-blue-600 text-white border-blue-600' : 'hover:bg-gray-50'">{{ page }}</button>
            </div>
            <button @click="goToPage(currentPage + 1)" :disabled="currentPage === totalPages" class="px-4 py-2 border rounded-md" :class="currentPage === totalPages ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'hover:bg-gray-50 text-blue-600'">Tiếp &raquo;</button>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>