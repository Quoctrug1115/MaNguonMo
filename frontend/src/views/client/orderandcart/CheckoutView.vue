<script setup>
import { ref, computed, onMounted } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

const router = useRouter()
const cartItems = ref([])
const token = localStorage.getItem('token')

// Form dữ liệu khách hàng sẽ điền
const form = ref({
  fullName: '',
  address: '',
  city: '',
  phone: '',
  email: ''
})

// Lấy giỏ hàng (giống hệt trang Cart)
const fetchCart = async () => {
  if (!token) {
    router.push('/login')
    return
  }
  try {
    const res = await axios.get('http://localhost:3000/api/cart', {
      headers: { 'Authorization': `Bearer ${token}` }
    })
    cartItems.value = res.data.data
  } catch (error) {
    console.error("Lỗi lấy giỏ hàng:", error)
  }
}

onMounted(() => { fetchCart() })

// Tính tổng tiền tự động
const subtotal = computed(() => {
  return cartItems.value.reduce((total, item) => total + (item.price * item.quantity), 0)
})

// Hàm format tiền
const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price || 0)
}

// Hàm XỬ LÝ ĐẶT HÀNG
const placeOrder = async () => {
  // 1. Kiểm tra điền thiếu
  if (!form.value.address || !form.value.phone) {
    alert("Vui lòng điền Địa chỉ và Số điện thoại!")
    return
  }

  try {
    // 2. Gọi API Checkout (XÓA BỎ DÒNG user_id)
    const res = await axios.post('http://localhost:3000/api/orders/checkout', {
      shipping_address: `${form.value.address}, ${form.value.city}`,
      phone_number: form.value.phone
    }, {
      headers: { 'Authorization': `Bearer ${token}` }
    })

    // 3. Xử lý khi thành công
    if (res.status === 200) {
      alert("🎉 " + res.data.message + "\nMã đơn hàng: " + res.data.order_id)
      router.push('/') // Đẩy về trang chủ
    }

  } catch (error) {
    console.error("Lỗi đặt hàng:", error)
    alert(error.response?.data?.error || "Đặt hàng thất bại. Vui lòng thử lại!")
  }
}
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
          <input v-model="form.fullName" type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>
        
        <div>
          <label class="block text-sm text-gray-500 mb-2">Company Name</label>
          <input type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Địa chỉ<span class="text-red-500">*</span></label>
          <input v-model="form.address" type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Quốc Gia (optional)</label>
          <input type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Thành Phố<span class="text-red-500">*</span></label>
          <input v-model="form.city" type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
        </div>

        <div>
          <label class="block text-sm text-gray-500 mb-2">Số Điện Thoại<span class="text-red-500">*</span></label>
          <input v-model="form.phone" type="text" class="w-full bg-gray-100 rounded-md px-4 py-3 outline-none focus:ring-1 focus:ring-blue-500" />
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
          <div v-for="item in cartItems" :key="item.cart_item_id" class="flex justify-between items-center">
            <div class="flex items-center gap-4">
              <img :src="item.image_url" class="w-10 h-10 object-cover rounded" alt="img" />
              <span class="font-medium">{{ item.product_name }} (x{{ item.quantity }})</span>
            </div>
            <span class="font-medium text-red-500">{{ formatPrice(item.price * item.quantity) }}</span>
          </div>
        </div>

        <div class="flex justify-between border-b border-gray-200 pb-4 mb-4">
          <span class="text-gray-600">Sản Phẩm:</span>
          <span class="font-medium">{{ formatPrice(subtotal) }}</span>
        </div>
        <div class="flex justify-between border-b border-gray-200 pb-4 mb-4">
          <span class="text-gray-600">Phí Vận Chuyển:</span>
          <span class="font-medium">Free</span>
        </div>
        <div class="flex justify-between mb-8">
          <span class="text-gray-800 font-medium">Thành Tiền:</span>
          <span class="font-bold text-lg">{{ formatPrice(subtotal) }}</span>
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

        <button @click="placeOrder" class="bg-blue-600 text-white px-10 py-4 rounded font-medium hover:bg-blue-700 transition w-full md:w-auto">
          Xác nhận đặt hàng
        </button>
      </div>
    </div>
  </div>
</template>