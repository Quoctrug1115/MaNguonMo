<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

const router = useRouter()
const orders = ref([])
const isLoading = ref(true)

const token = localStorage.getItem('token')
// Hàm lấy dữ liệu
const fetchOrders = async () => {
  if (!token) {
    router.push('/login')
    return
  }

  try {
    const res = await axios.get(`${import.meta.env.VITE_API_BASE_URL}/orders/user`, {
      headers: { 'Authorization': `Bearer ${token}` }
    })
    orders.value = res.data.data
  } catch (error) {
    console.error("Lỗi lấy đơn hàng:", error)
  } finally {
    isLoading.value = false
  }
}

onMounted(() => { fetchOrders() })

// Các hàm tiện ích (Format Tiền, Ngày tháng, và Dịch Trạng thái)
const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price || 0)
}

const formatDate = (dateString) => {
  const options = { year: 'numeric', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' };
  return new Date(dateString).toLocaleDateString('vi-VN', options);
}

const getOrderStatusText = (status) => {
  const statusMap = {
    'pending': 'Chờ xác nhận',
    'shipping': 'Đang giao hàng',
    'completed': 'Đã hoàn thành',
    'cancelled': 'Đã hủy'
  }
  // Nếu status khớp với map thì trả về tiếng Việt, nếu không thì báo 'Không rõ'
  return statusMap[status] || 'Không rõ'
}

// (Bonus) Hàm này giúp bạn đổi màu chữ/nền tùy theo trạng thái cho đẹp mắt
const getOrderStatusClass = (status) => {
  const classMap = {
    'pending': 'text-yellow-600 bg-yellow-100',
    'shipping': 'text-blue-600 bg-blue-100',
    'completed': 'text-green-600 bg-green-100',
    'cancelled': 'text-red-600 bg-red-100'
  }
  return classMap[status] || 'text-gray-600 bg-gray-100'
}
</script>

<template>
  <div class="container mx-auto px-4 max-w-5xl py-10 mb-20 text-gray-800">
    <h1 class="text-3xl font-bold mb-8">Lịch sử đơn hàng</h1>

    <div v-if="isLoading" class="text-center py-20 text-gray-500">Đang tải dữ liệu...</div>
    
    <div v-else-if="orders.length === 0" class="text-center py-20 border border-gray-200 rounded-lg bg-gray-50">
      <p class="text-xl text-gray-600 mb-4">Bạn chưa có đơn hàng nào.</p>
      <router-link to="/" class="bg-blue-600 text-white px-6 py-2 rounded font-medium hover:bg-blue-700">Mua sắm ngay</router-link>
    </div>

    <div v-else class="flex flex-col gap-8">
      <div v-for="order in orders" :key="order.id" class="border border-gray-200 rounded-lg shadow-sm bg-white overflow-hidden">
        
        <div class="bg-gray-50 px-6 py-4 border-b border-gray-200 flex flex-col sm:flex-row justify-between sm:items-center gap-4">
          <div>
            <p class="text-sm text-gray-500">Mã đơn: <span class="font-mono text-black font-medium">{{ order.id }}</span></p>
            <p class="text-sm text-gray-500 mt-1">Ngày đặt: <span class="text-black">{{ formatDate(order.created_at) }}</span></p>
          </div>
          <div class="flex flex-col sm:items-end">
            <span class="px-3 py-1 text-sm font-semibold rounded-full" :class="getOrderStatusClass(order.status)">{{ getOrderStatusText(order.status) }}
            </span>
            <p class="text-lg font-bold text-red-500 mt-2">Tổng: {{ formatPrice(order.total_price) }}</p>
          </div>
        </div>

        <div class="p-6">
          <div class="mb-4 text-sm text-gray-600 bg-gray-50 p-4 rounded-md">
            <p><strong>Giao đến:</strong> {{ order.shipping_address }}</p>
            <p><strong>SĐT:</strong> {{ order.phone_number }}</p>
          </div>

          <div class="flex flex-col gap-4">
            <div v-for="(item, index) in order.items" :key="index" class="flex items-center gap-4 border-b border-gray-100 pb-4 last:border-0 last:pb-0">
              <img :src="item.image_url" class="w-16 h-16 object-cover rounded border border-gray-200" alt="product" />
              <div class="flex-grow">
                <p class="font-medium text-gray-900">{{ item.product_name }}</p>
                <p class="text-sm text-gray-500">x{{ item.quantity }}</p>
              </div>
              <div class="font-medium text-gray-700 text-right w-24">
                {{ formatPrice(item.price) }}
              </div>
            </div>
          </div>
        </div>
        
      </div>
    </div>
  </div>
</template>