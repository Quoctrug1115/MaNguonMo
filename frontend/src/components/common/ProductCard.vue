<script setup>
import { computed } from 'vue'

// Khai báo props để nhận dữ liệu từ component cha
const props = defineProps({
  product: {
    type: Object,
    required: true,
    default: () => ({
      id: 1,
      name: 'Tên sản phẩm mặc định',
      image: '',
      price: 0,
      originalPrice: 0,
      discount: 0,
      isNew: false,
      rating: 0,
      reviews: 0    
    })
  }
})

// Format tiền tệ VNĐ
const formattedPrice = computed(() => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(props.product.price)
})

const formattedOriginalPrice = computed(() => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(props.product.originalPrice)
})

// Khai báo các hàm xử lý sự kiện (để tránh lỗi Vue warning)
const addToWishlist = () => {
  console.log('Thêm vào yêu thích:', props.product.id)
}

const viewProductDetails = () => {
  console.log('Xem nhanh sản phẩm:', props.product.id)
}

const addToCart = () => {
  console.log('Thêm vào giỏ hàng:', props.product.id)
}
</script>

<template>
  <!-- Card Container Mở -->
  <div class="group relative bg-white border border-gray-100 rounded-lg p-4 hover:shadow-xl transition-all duration-300 flex flex-col h-full">
    
    <!-- Phần Hình ảnh & Badges (Được bọc bởi router-link) -->
    <router-link :to="`/product/${product.id}`" class="relative h-48 bg-[#f5f5f5] rounded-md flex items-center justify-center mb-4 overflow-hidden block">
      
      <!-- Badge: Giảm giá hoặc Mới -->
      <div class="absolute top-3 left-3 flex flex-col gap-2 z-10">
        <span v-if="product.discount" class="bg-danger text-white text-xs font-bold px-2 py-1 rounded">
          -{{ product.discount }}%
        </span>
        <span v-if="product.isNew" class="bg-green-500 text-white text-xs font-bold px-2 py-1 rounded">
          NEW
        </span>
      </div>

      <!-- Actions: Yêu thích & Xem nhanh -->
      <div class="absolute top-3 right-3 flex flex-col gap-2 z-20 opacity-0 group-hover:opacity-100 transition-opacity duration-300">
        <button @click.prevent="addToWishlist" class="bg-white p-1.5 rounded-full shadow hover:bg-gray-100 text-gray-600 transition">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
        </button>
        <button @click.prevent="viewProductDetails" class="bg-white p-1.5 rounded-full shadow hover:bg-gray-100 text-gray-600 transition">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" /></svg>
        </button>
      </div>

      <!-- Ảnh sản phẩm -->
      <img v-if="product.image" :src="product.image" :alt="product.name" class="max-h-full object-contain group-hover:scale-105 transition-transform duration-500" />
      <span v-else class="text-gray-400 text-sm">Hình ảnh</span>

      <!-- Nút Thêm vào giỏ hàng (Đã thêm @click.prevent) -->
      <div class="absolute bottom-0 left-0 w-full translate-y-full group-hover:translate-y-0 transition-transform duration-300 z-20">
        <button @click.prevent="addToCart" class="w-full bg-black text-white py-2 text-sm font-medium hover:bg-gray-800 transition-colors">
          Thêm vào giỏ hàng
        </button>
      </div>
    </router-link>
    <!-- ĐÃ XÓA THẺ </div> THỪA Ở ĐÂY -->

    <!-- Phần Thông tin sản phẩm -->
    <div class="flex flex-col flex-grow">
      
      <!-- Tiêu đề (Được bọc bởi router-link) -->
      <router-link :to="`/product/${product.id}`" class="hover:text-primary transition-colors">
        <h3 class="text-sm font-medium text-gray-900 mb-2 line-clamp-2" :title="product.name">
          {{ product.name }}
        </h3>
      </router-link>
      
      <div class="mt-auto">
        <!-- Giá cả -->
        <div class="flex items-center gap-3 mb-2">
          <span class="text-danger font-bold">{{ formattedPrice }}</span>
          <span v-if="product.originalPrice > product.price" class="text-gray-400 text-xs line-through">
            {{ formattedOriginalPrice }}
          </span>
        </div>
        
        <!-- Đánh giá (Rating) -->
        <div class="flex items-center gap-1">
          <div class="flex text-yellow-400">
            <svg v-for="i in 5" :key="i" class="w-4 h-4" :class="i <= product.rating ? 'fill-current' : 'text-gray-300 fill-current'" viewBox="0 0 24 24">
              <path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/>
            </svg>
          </div>
          <span class="text-xs text-gray-500 ml-1">({{ product.reviews }})</span>
        </div>
      </div>
    </div>
    
  <!-- Card Container Đóng -->
  </div>
</template>