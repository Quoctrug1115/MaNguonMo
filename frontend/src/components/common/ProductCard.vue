<script setup>
import { computed } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'
import { fetchCartCount } from '@/store/cartState'
import { likedProductIds, fetchWishlist } from '@/store/wishlistState'

const router = useRouter()

// Khai báo props để nhận dữ liệu từ component cha
const props = defineProps({
  product: {
    type: Object,
    required: true,
    default: () => ({
      id: '',
      name: 'Tên sản phẩm mặc định',
      image_url: '',
      price: 0,
      original_price: 0,
      discount_percent: 0,
      is_new: false,
      rating: 0,
      reviews_count: 0
    })
  }
})

// Format tiền tệ VNĐ
const formattedPrice = computed(() => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(props.product.price || 0)
})

const formattedOriginalPrice = computed(() => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(props.product.original_price || 0)
})

// 3. Hàm Thêm vào giỏ hàng (ĐÃ FIX LỖI TOKEN & USER_ID)
const handleAddToCart = async () => {
  const token = localStorage.getItem('token')

  if (!token) {
    alert('Vui lòng đăng nhập để thêm sản phẩm vào giỏ hàng!')
    router.push('/login')
    return
  }

  try {
    const response = await axios.post('http://localhost:3000/api/cart', 
      {
        // Xóa user_id đi vì Backend tự lấy từ Token
        product_id: props.product.id, 
        quantity: 1 
      },
      {
        // BẮT BUỘC CÓ: Gửi Token qua Header cho Backend
        headers: {
          'Authorization': `Bearer ${token}`
        }
      }
    )

    if (response.status === 200 || response.status === 201) {
      alert(`🛒 Đã thêm [${props.product.name}] vào giỏ hàng thành công!`)
      fetchCartCount()
    }
  } catch (error) {
    if (error.response?.status === 401) {
      alert('Phiên đăng nhập hết hạn, vui lòng đăng nhập lại!')
      router.push('/login')
    } else {
      console.error('Lỗi khi thêm vào giỏ hàng:', error)
      alert('Có lỗi xảy ra, không thể thêm vào giỏ hàng lúc này.')
    }
  }
}

const isLiked = computed(() => {
  return likedProductIds.value.includes(props.product.id)
})

// 4. Hàm Thêm/Xóa Yêu Thích (ĐÃ BỔ SUNG TOKEN TRÁNH LỖI TƯƠNG TỰ)
const toggleWishlist = async () => {
  const userStr = localStorage.getItem('user')
  const token = localStorage.getItem('token')

  if (!userStr || !token) {
    alert('Vui lòng đăng nhập để sử dụng tính năng này!')
    router.push('/login')
    return
  }

  const user = JSON.parse(userStr)
  
  // Cấu hình Header chứa Token dùng chung cho cả gọi POST và DELETE
  const config = {
    headers: { 'Authorization': `Bearer ${token}` }
  }

  try {
    if (isLiked.value) {
      // Thực hiện XÓA (Truyền header vào tham số thứ 2 của axios.delete)
      await axios.delete(`http://localhost:3000/api/wishlist/${user.id}/${props.product.id}`, config)
    } else {
      // Thực hiện THÊM (Truyền header vào tham số thứ 3 của axios.post)
      await axios.post('http://localhost:3000/api/wishlist', {
        user_id: user.id,
        product_id: props.product.id
      }, config)
    }
    
    // Yêu cầu trạm phát sóng cập nhật lại dữ liệu ngay lập tức
    fetchWishlist()
    
  } catch (error) {
    console.error("Lỗi thao tác Wishlist:", error)
  }
}
</script>

<template>
  <!-- Card Container Mở -->
  <div class="group relative bg-white border border-gray-100 rounded-lg p-4 hover:shadow-xl transition-all duration-300 flex flex-col h-full">

    <!-- Phần Hình ảnh & Badges (Được bọc bởi router-link) -->
    <router-link :to="`/product/${product.id}`" class="relative h-48 bg-[#f5f5f5] rounded-md flex items-center justify-center mb-4 overflow-hidden block">

      <!-- Badge: Giảm giá hoặc Mới -->
      <div class="absolute top-3 left-3 flex flex-col gap-2 z-10">
        <!-- Đã sửa thành discount_percent -->
        <span v-if="product.discount_percent" class="bg-danger text-white text-xs font-bold px-2 py-1 rounded">
          -{{ product.discount_percent }}%
        </span>
        <!-- Đã sửa thành is_new -->
        <span v-if="product.is_new" class="bg-green-500 text-white text-xs font-bold px-2 py-1 rounded">
          NEW
        </span>
      </div>

      <!-- Actions: Yêu thích & Xem nhanh -->
        <div class="absolute top-3 right-3 flex flex-col gap-2 z-20 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
          
          <button @click.prevent="toggleWishlist" class="bg-white p-1.5 rounded-full shadow hover:bg-gray-100 transition">
            <svg class="w-5 h-5 transition-colors" :class="isLiked ? 'text-red-500 fill-current' : 'text-gray-600'" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
            </svg>
          </button>

          <button @click.prevent="viewProductDetails" class="bg-white p-1.5 rounded-full shadow hover:bg-gray-100 text-gray-600 transition">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" /></svg>
          </button>
          
        </div>

      <!-- Ảnh sản phẩm (Đã sửa thành image_url) -->
      <img v-if="product.image_url" :src="product.image_url" :alt="product.name" class="max-h-full object-contain group-hover:scale-105 transition-transform duration-500" />
      <span v-else class="text-gray-400 text-sm">Hình ảnh</span>

      <!-- Nút Thêm vào giỏ hàng -->
      <div class="absolute bottom-0 left-0 w-full translate-y-full group-hover:translate-y-0 transition-transform duration-300 z-20">
        <button @click.prevent="handleAddToCart" class="w-full bg-black text-white py-2 text-sm font-medium hover:bg-gray-800 transition-colors">
          Thêm vào giỏ hàng
        </button>
      </div>
    </router-link>

    <!-- Phần Thông tin sản phẩm -->
    <div class="flex flex-col flex-grow">

      <!-- Tiêu đề -->
      <router-link :to="`/product/${product.id}`" class="hover:text-primary transition-colors">
        <h3 class="text-sm font-medium text-gray-900 mb-2 line-clamp-2" :title="product.name">
          {{ product.name }}
        </h3>
      </router-link>

      <div class="mt-auto">
        <!-- Giá cả -->
        <div class="flex items-center gap-3 mb-2">
          <span class="text-danger font-bold">{{ formattedPrice }}</span>
          <!-- Đã sửa thành original_price -->
          <span v-if="product.original_price && product.original_price > product.price" class="text-gray-400 text-xs line-through">
            {{ formattedOriginalPrice }}
          </span>
        </div>

        <!-- Đánh giá (Rating) -->
        <div class="flex items-center gap-1">
          <div class="flex text-yellow-400">
            <svg v-for="i in 5" :key="i" class="w-4 h-4" :class="i <= (product.rating || 0) ? 'fill-current' : 'text-gray-300 fill-current'" viewBox="0 0 24 24">
              <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/>
            </svg>
          </div>
          <!-- Đã sửa thành reviews_count -->
          <span class="text-xs text-gray-500 ml-1">({{ product.reviews_count || 0 }})</span>
        </div>
      </div>
    </div>

  </div>
</template>