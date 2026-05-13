<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'

const orders = ref([])
const isLoading = ref(true)

// Hàm lấy dữ liệu toàn bộ đơn hàng
const fetchOrders = async () => {
  try {
    isLoading.value = true
    const token = localStorage.getItem('token')
    
    const res = await axios.get('http://localhost:3000/api/admin/orders', {
      headers: { Authorization: `Bearer ${token}` }
    })
    orders.value = res.data.data
  } catch (error) {
    console.error("Lỗi lấy đơn hàng:", error)
    alert("Không thể tải danh sách đơn hàng!")
  } finally {
    isLoading.value = false
  }
}

// Hàm cập nhật trạng thái khi Admin đổi Select Box
const handleStatusChange = async (orderId, newStatus) => {
  try {
    const token = localStorage.getItem('token')
    
    await axios.put(`http://localhost:3000/api/admin/orders/${orderId}/status`, 
      { status: newStatus },
      { headers: { Authorization: `Bearer ${token}` } }
    )
    
    alert('✅ Cập nhật trạng thái thành công!')
  } catch (error) {
    alert('Lỗi khi cập nhật trạng thái. Vui lòng thử lại.')
    fetchOrders() // Nếu lỗi thì load lại dữ liệu cũ
  }
}

// Format tiền tệ
const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price)
}

// Format ngày tháng
const formatDate = (dateString) => {
  if (!dateString) return ''
  const date = new Date(dateString)
  return new Intl.DateTimeFormat('vi-VN', { 
    day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit' 
  }).format(date)
}

onMounted(() => {
  fetchOrders()
})
</script>

<template>
  <div class="p-8 max-w-7xl mx-auto">
    <div class="flex justify-between items-center mb-8">
      <h2 class="text-[28px] font-bold text-gray-800 tracking-wide">Quản Lý Đơn Hàng</h2>
      <button @click="fetchOrders" class="px-4 py-2 bg-white border border-gray-200 rounded-lg shadow-sm hover:bg-gray-50 flex items-center gap-2 font-semibold text-gray-700">
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
        Làm mới
      </button>
    </div>

    <!-- Bảng hiển thị Đơn Hàng -->
    <div class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
      <div v-if="isLoading" class="p-8 text-center text-gray-500 font-semibold">
        Đang tải dữ liệu đơn hàng...
      </div>
      
      <div v-else-if="orders.length === 0" class="p-8 text-center text-gray-500 font-semibold">
        Chưa có đơn hàng nào trong hệ thống!
      </div>

      <table v-else class="w-full text-left border-collapse">
        <thead>
          <tr class="bg-gray-50 border-b border-gray-100 text-gray-500 text-sm">
            <th class="py-4 px-6 font-semibold">Mã Đơn</th>
            <th class="py-4 px-6 font-semibold">Khách Hàng</th>
            <th class="py-4 px-6 font-semibold">Chi Tiết Sản Phẩm</th>
            <th class="py-4 px-6 font-semibold">Tổng Tiền</th>
            <th class="py-4 px-6 font-semibold">Ngày Đặt</th>
            <th class="py-4 px-6 font-semibold text-center">Trạng Thái</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-100">
          <tr v-for="order in orders" :key="order.id" class="hover:bg-gray-50 transition-colors">
            
            <!-- Mã đơn (rút gọn) -->
            <td class="py-4 px-6 text-sm font-mono text-gray-600">
              #{{ order.id.split('-')[0].toUpperCase() }}
            </td>
            
            <!-- Khách hàng -->
            <td class="py-4 px-6">
              <div class="font-bold text-gray-800 text-sm">{{ order.customer_name }}</div>
              <div class="text-xs text-gray-500">{{ order.phone_number }}</div>
            </td>
            
            <!-- Chi tiết món hàng -->
            <td class="py-4 px-6">
              <ul class="text-xs text-gray-600 list-disc pl-4">
                <li v-for="(item, idx) in order.items" :key="idx" class="mb-1">
                  <span class="font-semibold">{{ item.product_name }}</span> (x{{ item.quantity }})
                </li>
              </ul>
            </td>

            <!-- Tổng tiền -->
            <td class="py-4 px-6 text-sm font-bold text-red-600">
              {{ formatPrice(order.total_price) }}
            </td>

            <!-- Ngày đặt -->
            <td class="py-4 px-6 text-sm text-gray-500">
              {{ formatDate(order.created_at) }}
            </td>

            <!-- Cập nhật Trạng thái -->
            <td class="py-4 px-6 text-center">
              <select 
                v-model="order.status"
                @change="handleStatusChange(order.id, order.status)"
                class="px-3 py-1.5 text-sm font-semibold rounded-lg border outline-none cursor-pointer"
                :class="{
                  'bg-yellow-50 text-yellow-700 border-yellow-200': order.status === 'pending',
                  'bg-blue-50 text-blue-700 border-blue-200': order.status === 'shipping',
                  'bg-green-50 text-green-700 border-green-200': order.status === 'completed',
                  'bg-red-50 text-red-700 border-red-200': order.status === 'cancelled'
                }"
              >
                <option value="pending" class="bg-white text-black">Chờ xử lý</option>
                <option value="shipping" class="bg-white text-black">Đang giao</option>
                <option value="completed" class="bg-white text-black">Hoàn thành</option>
                <option value="cancelled" class="bg-white text-black">Đã hủy</option>
              </select>
            </td>
            
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>