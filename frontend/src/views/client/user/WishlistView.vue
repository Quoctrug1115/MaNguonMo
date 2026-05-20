<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'
import { fetchWishlist } from '@/store/wishlistState'
import { fetchCartCount } from '@/store/cartState'

const router = useRouter()
const wishlistItems = ref([])
const isLoading = ref(true)
const token = localStorage.getItem('token')


// 1. Tải danh sách yêu thích từ Backend
const loadWishlist = async () => {
  if (!token) {
    router.push('/login')
    return
  }

  try {
    const response = await axios.get(`${import.meta.env.VITE_API_BASE_URL}/wishlist`, {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
    wishlistItems.value = response.data.data
  } catch (error) {
    console.error("Lỗi tải danh sách yêu thích:", error)
  } finally {
    isLoading.value = false
  }
}

onMounted(() => { loadWishlist() })

// 2. Xóa khỏi danh sách yêu thích
const removeItem = async (productId) => {
  if (!token) {
    router.push('/login')
    return
  }
  try {
    await axios.delete(`${import.meta.env.VITE_API_BASE_URL}/wishlist/${productId}`, {
      headers: { Authorization: `Bearer ${token}` }
    })
    await fetchWishlist()
    // Báo cho Trạm phát sóng cập nhật lại số lượng cục đỏ
    fetchWishlist()
  } catch (error) {
    console.error("Lỗi xóa sản phẩm:", error)
  }
}

// 3. Thêm thẳng vào Giỏ hàng từ trang Yêu thích
const moveToCart = async (product) => {
  if (!token) {
    router.push('/login')
    return
  }
  try {
    const res = await axios.post(`${import.meta.env.VITE_API_BASE_URL}/cart`, {
      product_id: product.product_id,
      quantity: 1
    }, {
      headers: {
        Authorization: `Bearer ${token}`
      }
    })
    
    if (res.status === 200) {
      alert(`🛒 Đã thêm [${product.product_name}] vào giỏ hàng!`)
      fetchCartCount() // Báo Giỏ hàng nhảy số
      removeItem(product.product_id)
    }
  } catch (error) {
    console.error("Lỗi thêm vào giỏ hàng:", error)
    alert("Có lỗi xảy ra, không thể thêm vào giỏ hàng lúc này.")
  }
}

const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price || 0)
}
</script>

<template>
  <div class="container mx-auto px-4 max-w-5xl py-10 mb-20 text-gray-800">
    <div class="flex justify-between items-end mb-8">
      <h1 class="text-3xl font-bold">Mục Yêu Thích</h1>
      <span class="text-gray-500">Bạn đang có {{ wishlistItems.length }} sản phẩm</span>
    </div>

    <div v-if="isLoading" class="text-center py-20 text-gray-500">Đang tải dữ liệu...</div>
    
    <div v-else-if="wishlistItems.length === 0" class="text-center py-20 border border-gray-200 rounded-lg bg-gray-50">
      <svg class="w-16 h-16 mx-auto text-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
      <p class="text-xl text-gray-600 mb-4">Danh sách yêu thích của bạn đang trống.</p>
      <router-link to="/" class="bg-blue-600 text-white px-6 py-2 rounded font-medium hover:bg-blue-700">Tiếp tục mua sắm</router-link>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div 
        v-for="item in wishlistItems" :key="item.product_id" 
        class="flex items-center gap-4 p-4 border border-gray-200 rounded-lg shadow-sm bg-white relative group"
      >
        <button @click="removeItem(item.product_id)" class="absolute top-3 right-3 text-gray-400 hover:text-red-500 transition" title="Xóa khỏi danh sách">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
        </button>

        <img :src="item.image_url" class="w-24 h-24 object-cover rounded border border-gray-100" alt="product" />
        
        <div class="flex flex-col flex-grow">
          <h3 class="font-medium text-lg text-gray-900 truncate pr-6" :title="item.product_name">{{ item.product_name }}</h3>
          <p class="text-red-500 font-bold mt-1">{{ formatPrice(item.price) }}</p>
          
          <button @click="moveToCart(item)" class="mt-3 bg-black text-white px-4 py-2 text-sm font-medium rounded hover:bg-gray-800 transition w-max">
            Thêm vào giỏ hàng
          </button>
        </div>
      </div>
    </div>
  </div>
</template>