<script setup>
import { ref } from 'vue'
import ProductCard from '@/components/common/ProductCard.vue'

// Dữ liệu mô phỏng cho sản phẩm hiện tại
const product = ref({
  name: 'Havic HV G-92 Gamepad',
  price: 1920000, 
  rating: 4,
  reviews: 150,
  inStock: true,
  description: 'PlayStation 5 Controller Skin High quality vinyl with air channel adhesive for easy bubble free install & mess free removal Pressure sensitive.',
  // Giả lập danh sách ảnh
  images: ['', '', '', '']
})

// Trạng thái cho việc chọn Size và Số lượng
const sizes = ['XS', 'S', 'M', 'L', 'XL']
const selectedSize = ref('M')
const quantity = ref(2)

// Dữ liệu mô phỏng cho Sản phẩm liên quan (Related Items)
const relatedProducts = ref([
  { id: 11, name: 'Gamepad Đỏ', price: 1200000, originalPrice: 1600000, discount: 40, image: '', rating: 5, reviews: 88 },
  { id: 12, name: 'Bàn phím cơ RGB', price: 9600000, originalPrice: 11600000, discount: 35, image: '', rating: 4, reviews: 75 },
  { id: 13, name: 'Màn hình Gaming MSI', price: 3700000, originalPrice: 4000000, discount: 30, image: '', rating: 5, reviews: 99 },
  { id: 14, name: 'Tản nhiệt nước', price: 1600000, originalPrice: 1700000, discount: null, image: '', rating: 4, reviews: 65 }
])

// Format tiền tệ VNĐ
const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price)
}

const handleBuyNow = () => {
  console.log(`Đã mua ${quantity.value} sản phẩm size ${selectedSize.value}`)
  // TODO: Thêm vào giỏ hàng và chuyển hướng sang trang Checkout
}
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20">
    
    <!-- Breadcrumb -->
    <div class="text-sm text-gray-500 mb-10">
      <router-link to="/account" class="hover:text-primary transition">Account</router-link> / 
      <router-link to="/" class="hover:text-primary transition">Gaming</router-link> / 
      <span class="text-gray-900 font-medium">{{ product.name }}</span>
    </div>

    <!-- KHU VỰC CHI TIẾT SẢN PHẨM -->
    <div class="flex flex-col lg:flex-row gap-12 mb-24">
      
      <!-- Cột Trái: Bộ sưu tập hình ảnh -->
      <div class="lg:w-3/5 flex gap-6">
        <!-- List Ảnh nhỏ (Thumbnails) -->
        <div class="flex flex-col gap-4">
          <div 
            v-for="(img, index) in product.images" 
            :key="index"
            class="w-32 h-32 bg-[#f5f5f5] rounded-lg flex items-center justify-center cursor-pointer border-2 hover:border-primary transition-colors"
          >
            <!-- Thay bằng <img> thật sau này -->
            <span class="text-xs text-gray-400">Hình {{ index + 1 }}</span>
          </div>
        </div>
        
        <!-- Ảnh chính to -->
        <div class="flex-grow bg-[#f5f5f5] rounded-lg flex items-center justify-center min-h-[500px]">
           <span class="text-gray-400 font-medium">Hình ảnh chính Gamepad</span>
        </div>
      </div>

      <!-- Cột Phải: Thông tin & Hành động -->
      <div class="lg:w-2/5 flex flex-col justify-center">
        <h1 class="text-2xl font-bold text-gray-900 mb-3">{{ product.name }}</h1>
        
        <!-- Đánh giá & Tình trạng kho -->
        <div class="flex items-center gap-4 mb-4 text-sm">
          <div class="flex text-yellow-400">
            <svg v-for="i in 5" :key="i" class="w-4 h-4" :class="i <= product.rating ? 'fill-current' : 'text-gray-300 fill-current'" viewBox="0 0 24 24"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg>
          </div>
          <span class="text-gray-500">({{ product.reviews }} Reviews)</span>
          <span class="text-gray-300">|</span>
          <span class="text-green-500 font-medium">In Stock</span>
        </div>

        <!-- Giá tiền -->
        <div class="text-2xl font-medium text-gray-900 mb-6">
          {{ formatPrice(product.price) }}
        </div>

        <!-- Mô tả ngắn -->
        <p class="text-sm text-gray-600 mb-6 leading-relaxed">
          {{ product.description }}
        </p>

        <hr class="border-gray-200 mb-6" />

        <!-- Phân loại (Size) -->
        <div class="flex items-center gap-6 mb-6">
          <span class="text-gray-900 text-sm font-medium">Size:</span>
          <div class="flex gap-3">
            <button 
              v-for="size in sizes" 
              :key="size"
              @click="selectedSize = size"
              :class="[
                'w-8 h-8 rounded text-sm font-medium border flex items-center justify-center transition-colors',
                selectedSize === size ? 'bg-danger text-white border-danger' : 'border-gray-300 text-gray-900 hover:border-gray-900'
              ]"
            >
              {{ size }}
            </button>
          </div>
        </div>

        <!-- Số lượng & Nút Mua -->
        <div class="flex items-center gap-4 mb-10">
          
          <!-- Tăng giảm số lượng -->
          <div class="flex items-center border border-gray-300 rounded overflow-hidden h-11 w-32">
            <button @click="if(quantity > 1) quantity--" class="w-10 h-full hover:bg-gray-100 text-gray-600 transition flex items-center justify-center border-r border-gray-300">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"/></svg>
            </button>
            <input type="text" v-model.number="quantity" class="w-full h-full text-center text-base font-medium focus:outline-none" />
            <button @click="quantity++" class="w-10 h-full bg-danger hover:bg-red-600 text-white transition flex items-center justify-center">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/></svg>
            </button>
          </div>

          <!-- Mua Ngay -->
          <button @click="handleBuyNow" class="flex-grow bg-primary hover:bg-blue-600 text-white h-11 rounded font-medium transition">
            Mua ngay
          </button>

          <!-- Yêu thích -->
          <button class="w-11 h-11 border border-gray-300 rounded flex items-center justify-center hover:bg-gray-50 text-gray-700 transition">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
          </button>
        </div>

        <!-- Delivery & Return Box -->
        <div class="border border-gray-300 rounded divide-y divide-gray-300">
          <div class="p-4 flex gap-4 items-center">
            <svg class="w-8 h-8 text-gray-900" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M16 8h-6v6h6V8z" /></svg>
            <div>
              <h4 class="font-medium text-gray-900 text-sm mb-1">Free Delivery</h4>
              <a href="#" class="text-xs text-gray-600 underline hover:text-gray-900">Enter your postal code for Delivery Availability</a>
            </div>
          </div>
          <div class="p-4 flex gap-4 items-center">
            <svg class="w-8 h-8 text-gray-900" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" /></svg>
            <div>
              <h4 class="font-medium text-gray-900 text-sm mb-1">Return Delivery</h4>
              <p class="text-xs text-gray-600">Free 30 Days Delivery Returns. <a href="#" class="underline hover:text-gray-900">Details</a></p>
            </div>
          </div>
        </div>

      </div>
    </div>

    <!-- KHU VỰC RELATED ITEM -->
    <section>
      <div class="flex items-center gap-4 mb-10">
        <div class="w-4 h-8 bg-primary rounded-sm"></div>
        <h2 class="text-lg font-bold text-danger">Related Item</h2>
      </div>

      <!-- Tái sử dụng ProductCard component -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-6">
        <ProductCard v-for="item in relatedProducts" :key="item.id" :product="item" />
      </div>
    </section>

  </div>
</template>