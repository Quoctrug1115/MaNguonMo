<script setup>
import { ref, computed } from 'vue'

// Mock Data: Sản phẩm trong giỏ hàng
const cartItems = ref([
  { id: 1, name: 'LCD Monitor', price: 6500000, quantity: 1, image: '' },
  { id: 2, name: 'H1 Gamepad', price: 550000, quantity: 2, image: '' }
])

const couponCode = ref('')

// Format tiền tệ VNĐ
const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price)
}

// Xóa sản phẩm khỏi giỏ
const removeItem = (id) => {
  cartItems.value = cartItems.value.filter(item => item.id !== id)
}

// Tính tổng tiền đơn hàng
const cartTotal = computed(() => {
  return cartItems.value.reduce((total, item) => total + (item.price * item.quantity), 0)
})

// Thêm hàm này để xử lý giảm số lượng
const decreaseQuantity = (item) => {
  if (item.quantity > 1) {
    item.quantity--
  }
}
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20">
    
    <!-- Breadcrumb -->
    <div class="text-sm text-gray-500 mb-10">
      <router-link to="/" class="hover:text-primary">Home</router-link> / <span class="text-gray-900">Cart</span>
    </div>

    <!-- BẢNG GIỎ HÀNG -->
    <div class="bg-white rounded-lg shadow-sm border border-gray-200 overflow-hidden mb-8">
      <!-- Header Bảng -->
      <div class="grid grid-cols-12 gap-4 p-5 border-b border-gray-200 bg-gray-50 text-sm font-medium text-gray-700">
        <div class="col-span-5">Sản phẩm</div>
        <div class="col-span-2 text-center">Giá</div>
        <div class="col-span-3 text-center">Số lượng</div>
        <div class="col-span-2 text-right">Thành tiền</div>
      </div>

      <!-- Danh sách sản phẩm -->
      <div v-if="cartItems.length === 0" class="p-10 text-center text-gray-500">
        Giỏ hàng của bạn đang trống.
      </div>
      
      <div 
        v-for="item in cartItems" 
        :key="item.id" 
        class="grid grid-cols-12 gap-4 p-5 border-b border-gray-100 items-center hover:bg-gray-50 transition"
      >
        <!-- Cột 1: Thông tin sản phẩm -->
        <div class="col-span-5 flex items-center gap-4">
          <!-- Nút Xóa -->
          <button @click="removeItem(item.id)" class="text-danger hover:text-red-700 bg-red-50 p-1 rounded-full transition">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>
          </button>
          <!-- Ảnh -->
          <div class="w-16 h-16 bg-gray-200 rounded flex items-center justify-center text-xs text-gray-400 overflow-hidden">
             <img v-if="item.image" :src="item.image" :alt="item.name" class="object-cover w-full h-full" />
             <span v-else>Ảnh</span>
          </div>
          <span class="font-medium text-sm">{{ item.name }}</span>
        </div>

        <!-- Cột 2: Đơn giá -->
        <div class="col-span-2 text-center text-sm">
          {{ formatPrice(item.price) }}
        </div>

        <!-- Cột 3: Nút tăng giảm số lượng -->
        <div class="col-span-3 flex justify-center">
          <div class="flex items-center border border-gray-300 rounded overflow-hidden w-24">
            <button 
                @click="decreaseQuantity(item)" 
                class="px-2 py-1 bg-white hover:bg-gray-100 text-gray-600 transition"
                >
                -
            </button>
            <input 
              type="number" 
              v-model.number="item.quantity" 
              min="1" 
              class="w-full text-center text-sm py-1 focus:outline-none bg-white font-medium"
            />
            <button 
              @click="item.quantity++" 
              class="px-2 py-1 bg-white hover:bg-gray-100 text-gray-600 transition"
            >
              +
            </button>
          </div>
        </div>

        <!-- Cột 4: Thành tiền -->
        <div class="col-span-2 text-right text-sm font-medium">
          {{ formatPrice(item.price * item.quantity) }}
        </div>
      </div>
    </div>

    <!-- Các nút hành động (Trở về shop / Cập nhật giỏ hàng) -->
    <div class="flex justify-between items-center mb-16">
      <router-link to="/" class="border border-gray-300 px-8 py-3 rounded text-sm font-medium hover:bg-gray-50 transition">
        Trở về
      </router-link>
      <button class="border border-gray-300 px-8 py-3 rounded text-sm font-medium hover:bg-gray-50 transition">
        Update Cart
      </button>
    </div>

    <!-- KHU VỰC THANH TOÁN (Mã giảm giá & Tổng tiền) -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 items-start">
      
      <!-- Cột trái: Mã giảm giá -->
      <div class="flex gap-4">
        <input 
          type="text" 
          v-model="couponCode"
          placeholder="MÃ GIẢM GIÁ" 
          class="border border-gray-300 rounded px-6 py-3 w-64 focus:outline-none focus:border-primary text-sm"
        />
        <button class="bg-primary text-white px-8 py-3 rounded text-sm font-medium hover:bg-blue-600 transition">
          Thêm
        </button>
      </div>

      <!-- Cột phải: Box Tổng tiền -->
      <div class="border border-gray-900 rounded p-6">
        <h3 class="text-lg font-medium mb-6">Tổng tiền</h3>
        
        <div class="flex justify-between border-b border-gray-200 py-4 text-sm">
          <span class="text-gray-600">Đơn hàng:</span>
          <span class="font-medium">{{ formatPrice(cartTotal) }}</span>
        </div>
        
        <div class="flex justify-between border-b border-gray-200 py-4 text-sm">
          <span class="text-gray-600">Phí vận chuyển:</span>
          <span class="font-medium text-green-600">Free</span>
        </div>
        
        <div class="flex justify-between py-4 text-base font-medium">
          <span>Thành tiền:</span>
          <span>{{ formatPrice(cartTotal) }}</span>
        </div>

        <div class="mt-4 flex justify-center">
          <router-link to="/checkout" class="bg-primary text-white px-8 py-3 rounded text-sm font-medium hover:bg-blue-600 transition">
            Đi Đến Thanh Toán
          </router-link>
        </div>
      </div>

    </div>

  </div>
</template>

<style scoped>
/* Ẩn mũi tên lên xuống mặc định của input type="number" */
input[type="number"]::-webkit-inner-spin-button,
input[type="number"]::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
input[type="number"] {
  -moz-appearance: textfield;
}
</style>