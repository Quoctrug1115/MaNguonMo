<script setup>
import { ref } from 'vue'

// Dữ liệu mẫu
const cartItems = ref([
  { id: 1, name: 'LCD Monitor', price: 650, image: 'https://via.placeholder.com/40' },
  { id: 2, name: 'H1 Gamepad', price: 1100, image: 'https://via.placeholder.com/40' }
])

const subtotal = 1750
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20 text-gray-800">
    <nav class="text-sm text-gray-500 mb-10">
      <span class="hover:text-black cursor-pointer">Account</span> <span class="mx-2">/</span>
      <span class="hover:text-black cursor-pointer">My Account</span> <span class="mx-2">/</span>
      <span class="hover:text-black cursor-pointer">Product</span> <span class="mx-2">/</span>
      <router-link to="/cart" class="hover:text-black">View Cart</router-link> <span class="mx-2">/</span>
      <span class="text-black font-medium">CheckOut</span>
    </nav>

    <h1 class="text-3xl font-medium mb-10">Hóa đơn</h1>

    <div class="grid grid-cols-1 lg:grid-cols-12 gap-16">
      
      <div class="lg:col-span-6 flex flex-col gap-5">
        <div>
          <label class="block text-sm text-gray-500 mb-2">Họ và Tên<span class="text-red-500">*</span></label>
          <input type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>
        
        <div>
          <label class="block text-sm text-gray-500 mb-2">Company Name</label>
          <input type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Địa chỉ<span class="text-red-500">*</span></label>
          <input type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Quốc Gia (optional)</label>
          <input type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Thành Phố<span class="text-red-500">*</span></label>
          <input type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Số Điện Thoại<span class="text-red-500">*</span></label>
          <input type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Email<span class="text-red-500">*</span></label>
          <input type="email" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div class="flex items-center gap-3 mt-2">
          <input type="checkbox" id="save-info" class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500 cursor-pointer" checked />
          <label for="save-info" class="text-sm cursor-pointer">Save this information for faster check-out next time</label>
        </div>
      </div>

      <div class="lg:col-span-6 lg:pl-10">
        <div class="flex flex-col gap-6 mb-8">
          <div v-for="item in cartItems" :key="item.id" class="flex justify-between items-center">
            <div class="flex items-center gap-4">
              <img :src="item.image" class="w-10 h-10 object-cover rounded" alt="img" />
              <span class="font-medium">{{ item.name }}</span>
            </div>
            <span class="font-medium">${{ item.price }}</span>
          </div>
        </div>

        <div class="flex justify-between border-b border-gray-200 pb-4 mb-4">
          <span class="text-gray-600">Sản Phẩm:</span>
          <span class="font-medium">${{ subtotal }}</span>
        </div>
        <div class="flex justify-between border-b border-gray-200 pb-4 mb-4">
          <span class="text-gray-600">Phí Vận Chuyển:</span>
          <span class="font-medium">Free</span>
        </div>
        <div class="flex justify-between mb-8">
          <span class="text-gray-800 font-medium">Thành Tiền:</span>
          <span class="font-bold text-lg">${{ subtotal }}</span>
        </div>

        <div class="flex flex-col gap-4 mb-8">
          <div class="flex items-center justify-between">
            <label class="flex items-center gap-3 cursor-pointer">
              <input type="radio" name="payment" class="w-4 h-4 text-black focus:ring-black cursor-pointer" />
              <span>Bank</span>
            </label>
            <div class="flex gap-1">
              <div class="w-8 h-5 bg-gray-200 rounded text-[8px] flex items-center justify-center font-bold">VISA</div>
              <div class="w-8 h-5 bg-gray-200 rounded text-[8px] flex items-center justify-center font-bold">MC</div>
            </div>
          </div>
          <label class="flex items-center gap-3 cursor-pointer">
            <input type="radio" name="payment" class="w-4 h-4 text-black focus:ring-black cursor-pointer" checked />
            <span>Thanh Toán Khi Nhận Hàng</span>
          </label>
        </div>

        <div class="flex gap-4 mb-8">
          <input type="text" placeholder="Mã giảm giá" class="border border-black rounded px-4 py-3 flex-grow outline-none focus:ring-1 focus:ring-blue-500" />
          <button class="bg-blue-600 text-white px-8 py-3 rounded font-medium hover:bg-blue-700 transition">
            Thêm
          </button>
        </div>

        <button class="bg-blue-600 text-white px-10 py-4 rounded font-medium hover:bg-blue-700 transition w-full md:w-auto">
          Xác nhận đặt hàng
        </button>
      </div>

    </div>
  </div>
</template>