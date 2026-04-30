<script setup>
import { ref, computed } from 'vue'

// Quản lý dữ liệu form thông tin thanh toán
const billingForm = ref({
  fullName: '',
  companyName: '',
  address: '',
  country: '',
  city: '',
  phone: '',
  email: '',
  saveInfo: false
})

// Mock Data: Sản phẩm trong đơn hàng (Dựa theo dữ liệu từ Giỏ hàng)
const orderItems = ref([
  { id: 1, name: 'LCD Monitor', price: 6500000, image: '' },
  { id: 2, name: 'H1 Gamepad', price: 1100000, image: '' }
])

const paymentMethod = ref('cod') // 'bank' hoặc 'cod' (Thanh Toán Khi Nhận Hàng)
const couponCode = ref('')

// Format tiền tệ VNĐ
const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price)
}

// Tính tổng tiền sản phẩm
const subTotal = computed(() => {
  return orderItems.value.reduce((total, item) => total + item.price, 0)
})

// Tính tổng tiền cuối cùng (có thể cộng thêm phí ship sau này)
const total = computed(() => {
  return subTotal.value // Tạm thời Phí vận chuyển là Free
})

// Xử lý nút Đặt hàng
const placeOrder = () => {
  console.log('Thông tin giao hàng:', billingForm.value)
  console.log('Phương thức thanh toán:', paymentMethod.value)
  // TODO: Gọi API backend Rust Axum để tạo đơn hàng
}
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20">
    
    <!-- Breadcrumb -->
    <div class="text-sm text-gray-500 mb-12">
      <router-link to="/account" class="hover:text-primary">Account</router-link> / 
      <router-link to="/account" class="hover:text-primary">My Account</router-link> / 
      <router-link to="/" class="hover:text-primary">Product</router-link> / 
      <router-link to="/cart" class="hover:text-primary">View Cart</router-link> / 
      <span class="text-gray-900">CheckOut</span>
    </div>

    <h1 class="text-3xl font-bold mb-10 text-gray-900">Hóa đơn</h1>

    <div class="grid grid-cols-1 lg:grid-cols-12 gap-16">
      
      <!-- CỘT TRÁI: THÔNG TIN GIAO HÀNG -->
      <div class="lg:col-span-6">
        <form @submit.prevent="placeOrder">
          
          <div class="mb-6">
            <label class="block text-gray-500 text-sm mb-2">Họ và Tên <span class="text-danger">*</span></label>
            <input type="text" v-model="billingForm.fullName" required class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm" />
          </div>

          <div class="mb-6">
            <label class="block text-gray-500 text-sm mb-2">Company Name</label>
            <input type="text" v-model="billingForm.companyName" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm" />
          </div>

          <div class="mb-6">
            <label class="block text-gray-500 text-sm mb-2">Địa chỉ <span class="text-danger">*</span></label>
            <input type="text" v-model="billingForm.address" required class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm" />
          </div>

          <div class="mb-6">
            <label class="block text-gray-500 text-sm mb-2">Quốc Gia (optional)</label>
            <input type="text" v-model="billingForm.country" class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm" />
          </div>

          <div class="mb-6">
            <label class="block text-gray-500 text-sm mb-2">Thành Phố <span class="text-danger">*</span></label>
            <input type="text" v-model="billingForm.city" required class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm" />
          </div>

          <div class="mb-6">
            <label class="block text-gray-500 text-sm mb-2">Số Điện Thoại <span class="text-danger">*</span></label>
            <input type="tel" v-model="billingForm.phone" required class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm" />
          </div>

          <div class="mb-6">
            <label class="block text-gray-500 text-sm mb-2">Email <span class="text-danger">*</span></label>
            <input type="email" v-model="billingForm.email" required class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm" />
          </div>

          <div class="flex items-center gap-3 mt-8">
            <input type="checkbox" id="saveInfo" v-model="billingForm.saveInfo" class="w-4 h-4 text-primary bg-gray-100 border-gray-300 rounded focus:ring-primary" />
            <label for="saveInfo" class="text-sm text-gray-900">Save this information for faster check-out next time</label>
          </div>

        </form>
      </div>

      <!-- CỘT PHẢI: TÓM TẮT ĐƠN HÀNG & THANH TOÁN -->
      <div class="lg:col-span-6 lg:pl-10">
        
        <!-- Danh sách sản phẩm -->
        <div class="space-y-6 mb-8">
          <div v-for="item in orderItems" :key="item.id" class="flex items-center justify-between">
            <div class="flex items-center gap-4">
              <div class="w-12 h-12 bg-gray-100 rounded flex items-center justify-center text-xs text-gray-400 overflow-hidden">
                <img v-if="item.image" :src="item.image" :alt="item.name" class="object-cover w-full h-full" />
                <span v-else>Ảnh</span>
              </div>
              <span class="font-medium text-sm text-gray-900">{{ item.name }}</span>
            </div>
            <span class="text-sm font-medium">{{ formatPrice(item.price) }}</span>
          </div>
        </div>

        <!-- Bảng tính tiền -->
        <div class="space-y-4 border-b border-gray-200 pb-4 mb-4 text-sm text-gray-900">
          <div class="flex justify-between">
            <span>Sản Phẩm</span>
            <span>{{ formatPrice(subTotal) }}</span>
          </div>
          <div class="flex justify-between border-b border-gray-200 pb-4">
            <span>Phí Vận Chuyển:</span>
            <span class="text-green-600 font-medium">Free</span>
          </div>
          <div class="flex justify-between text-base">
            <span>Thành Tiền:</span>
            <span class="font-bold">{{ formatPrice(total) }}</span>
          </div>
        </div>

        <!-- Phương thức thanh toán -->
        <div class="space-y-4 mb-8">
          
          <label class="flex items-center justify-between cursor-pointer">
            <div class="flex items-center gap-3">
              <input type="radio" name="payment" value="bank" v-model="paymentMethod" class="w-4 h-4 text-primary focus:ring-primary" />
              <span class="text-sm font-medium">Bank</span>
            </div>
            <!-- Icon các ngân hàng (SVG placeholder) -->
            <div class="flex gap-1 opacity-70">
              <div class="w-8 h-5 bg-blue-100 border border-blue-200 rounded text-[8px] flex items-center justify-center text-blue-800 font-bold">VISA</div>
              <div class="w-8 h-5 bg-red-100 border border-red-200 rounded text-[8px] flex items-center justify-center text-red-800 font-bold">MC</div>
            </div>
          </label>

          <label class="flex items-center gap-3 cursor-pointer">
            <input type="radio" name="payment" value="cod" v-model="paymentMethod" class="w-4 h-4 text-primary focus:ring-primary" />
            <span class="text-sm font-medium">Thanh Toán Khi Nhận Hàng</span>
          </label>
        </div>

        <!-- Khối Mã giảm giá -->
        <div class="flex gap-4 mb-8">
          <input 
            type="text" 
            v-model="couponCode"
            placeholder="MÃ GIẢM GIÁ" 
            class="border border-gray-300 rounded px-4 py-3 flex-grow focus:outline-none focus:border-primary text-sm"
          />
          <button class="bg-primary text-white px-8 py-3 rounded text-sm font-medium hover:bg-blue-600 transition">
            Thêm
          </button>
        </div>

        <!-- Nút Đặt hàng -->
        <button 
          @click="placeOrder"
          class="bg-primary text-white px-10 py-4 rounded text-sm font-medium hover:bg-blue-600 transition"
        >
          Xác nhận đặt hàng
        </button>

      </div>

    </div>
  </div>
</template>