<script setup>
import { ref, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import ProductCard from '@/components/common/ProductCard.vue'
import SidebarMenu from '@/components/home/SidebarMenu.vue' // <--- Import component của bạn ở đây

const route = useRoute()
const router = useRouter()

const productList = ref([])
const isLoading = ref(true)
const currentCategory = ref(route.query.category || '')

const currentPage = ref(1)
const totalPages = ref(1)
const totalItems = ref(0)
const itemsPerPage = 12

// Hàm gọi API lấy sản phẩm
const fetchProducts = async (page, categoryId = '') => {
  isLoading.value = true
  try {
    let url = `http://localhost:3000/api/products?page=${page}&limit=${itemsPerPage}`
    if (categoryId) {
      url += `&category=${categoryId}`
    }

    const response = await fetch(url)
    const result = await response.json()

    if (response.ok) {
      productList.value = result.data
      currentPage.value = result.pagination.current_page
      totalPages.value = result.pagination.total_pages
      totalItems.value = result.pagination.total_items
    }
  } catch (error) {
    console.error('Lỗi lấy sản phẩm:', error)
  } finally {
    isLoading.value = false
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }
}

// Chuyển trang
const goToPage = (page) => {
  if (page < 1 || page > totalPages.value || page === currentPage.value) return
  router.push({ path: '/products', query: { ...route.query, page: page } })
}

// Vue sẽ tự động theo dõi URL. Hễ URL đổi (do bấm menu), nó sẽ tải lại Data
watch(() => route.query, (newQuery) => {
  const pageNum = parseInt(newQuery.page) || 1
  currentCategory.value = newQuery.category || ''
  fetchProducts(pageNum, currentCategory.value)
}, { deep: true })

onMounted(() => {
  const initialPage = parseInt(route.query.page) || 1
  fetchProducts(initialPage, currentCategory.value)
})
</script>

<template>
  <div class="container mx-auto px-4 max-w-7xl py-10 mb-20">

    <div class="mb-8 border-b pb-4">
      <h1 class="text-3xl font-bold text-gray-900">Tất Cả Sản Phẩm</h1>
    </div>

    <div class="flex flex-col lg:flex-row gap-8">

      <!-- CỘT TRÁI: DÙNG COMPONENT CỦA BẠN -->
      <div class="w-full lg:w-1/4">
        <div class="bg-white rounded-lg sticky top-6">
          <SidebarMenu />
        </div>
      </div>

      <!-- CỘT PHẢI: LƯỚI SẢN PHẨM -->
      <div class="w-full lg:w-3/4">

        <div v-if="isLoading" class="flex justify-center items-center py-32">
          <div class="animate-spin rounded-full h-10 w-10 border-t-2 border-b-2 border-primary"></div>
        </div>

        <div v-else>
          <div class="grid grid-cols-2 md:grid-cols-3 gap-6">
            <ProductCard v-for="product in productList" :key="product.id" :product="product" />
          </div>

          <div v-if="productList.length === 0" class="text-center py-20 text-gray-500">
            Không tìm thấy sản phẩm nào trong danh mục này.
          </div>

          <!-- PHÂN TRANG -->
          <div v-if="totalPages > 1" class="flex justify-center items-center gap-2 mt-12">
            <button @click="goToPage(currentPage - 1)" :disabled="currentPage === 1" class="px-4 py-2 border rounded-md" :class="currentPage === 1 ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'hover:bg-gray-50 hover:text-primary'">&laquo; Trước</button>
            <div class="hidden sm:flex gap-1">
              <button v-for="page in totalPages" :key="page" @click="goToPage(page)" class="w-10 h-10 border rounded-md flex items-center justify-center font-medium" :class="page === currentPage ? 'bg-primary text-white border-primary' : 'hover:bg-gray-50'">{{ page }}</button>
            </div>
            <button @click="goToPage(currentPage + 1)" :disabled="currentPage === totalPages" class="px-4 py-2 border rounded-md" :class="currentPage === totalPages ? 'bg-gray-100 text-gray-400 cursor-not-allowed' : 'hover:bg-gray-50 hover:text-primary'">Tiếp &raquo;</button>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>