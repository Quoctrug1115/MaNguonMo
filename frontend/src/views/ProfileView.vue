<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'

// State lưu trữ dữ liệu người dùng
const user = ref({
  first_name: '',
  last_name: '',
  email: '',
  address: ''
})

const password = ref({
  old: '',
  new: '',
  confirm: ''
})

// Hàm cấu hình Header có Token
const getAuthHeaders = () => {
  const token = localStorage.getItem('token') // Nhớ đổi tên key token cho đúng với dự án của bạn
  return { headers: { Authorization: `Bearer ${token}` } }
}

// Lấy thông tin cá nhân lúc vừa vào trang
const fetchProfile = async () => {
  try {
    const res = await axios.get('http://localhost:3000/api/profile', getAuthHeaders())
    // Giả sử API trả về các field này
    if (res.data) {
      user.value = {
        first_name: res.data.first_name || '',
        last_name: res.data.last_name || '',
        email: res.data.email || '',
        address: res.data.address || ''
      }
    }
  } catch (error) {
    console.error("Lỗi lấy thông tin:", error)
  }
}

// Lưu thay đổi
const handleSave = async () => {
  try {
    await axios.put('http://localhost:3000/api/profile', {
      first_name: user.value.first_name,
      last_name: user.value.last_name,
      email: user.value.email,
      address: user.value.address,
      // Truyền thêm password nếu bạn có logic đổi mật khẩu ở Backend
    }, getAuthHeaders())
    
    alert('Cập nhật hồ sơ thành công!')
    password.value = { old: '', new: '', confirm: '' } // Xóa trắng ô password sau khi lưu
  } catch (error) {
    console.error("Lỗi cập nhật:", error)
    alert('Có lỗi xảy ra khi cập nhật!')
  }
}

onMounted(() => {
  fetchProfile()
})
</script>

<template>
  <div class="bg-white min-h-screen text-gray-800 font-sans">
    
    <div class="container mx-auto px-4 max-w-6xl py-8">
      <div class="flex justify-between items-center text-sm mb-12">
        <div class="text-gray-500">
          Home / <span class="text-black font-medium">My Account</span>
        </div>
        <div>
          Xin Chào! <span class="text-red-500 font-medium">{{ user.last_name }} {{ user.first_name }}</span>
        </div>
      </div>

      <div class="flex flex-col lg:flex-row gap-12">
        
        <aside class="w-full lg:w-1/4">
          <div class="space-y-6">
            
            <div>
              <h3 class="font-bold text-gray-900 mb-3">Quản lý tài khoản</h3>
              <ul class="ml-6 space-y-2 text-sm text-gray-500">
                <li class="text-red-500 font-medium cursor-pointer">Tài khoản</li>
                <li class="hover:text-red-500 cursor-pointer transition-colors">địa chỉ nhận hàng</li>
                <li class="hover:text-red-500 cursor-pointer transition-colors">Phương thức thanh toán</li>
              </ul>
            </div>
            
            <div>
              <h3 class="font-bold text-gray-900 mb-3 mt-6">Đơn hàng của tôi</h3>
              <ul class="ml-6 space-y-2 text-sm text-gray-500">
                <li class="hover:text-red-500 cursor-pointer transition-colors">My Returns</li>
                <li class="hover:text-red-500 cursor-pointer transition-colors">My Cancellations</li>
              </ul>
            </div>

            <div>
              <h3 class="font-bold text-gray-900 mt-6 cursor-pointer hover:text-red-500 transition-colors">
                My WishList
              </h3>
            </div>
            
          </div>
        </aside>

        <main class="w-full lg:w-3/4">
          <div class="bg-white px-8 py-10 shadow-[0_0_15px_rgba(0,0,0,0.05)] rounded border border-gray-50">
            <h2 class="text-red-500 font-bold text-xl mb-6">Chỉnh sửa</h2>
            
            <form @submit.prevent="handleSave">
              
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                <div>
                  <label class="block text-sm mb-2 text-gray-700">Họ</label>
                  <input v-model="user.last_name" type="text" class="w-full bg-gray-100 border-none rounded p-3 text-sm outline-none focus:ring-1 focus:ring-gray-300" />
                </div>
                <div>
                  <label class="block text-sm mb-2 text-gray-700">Tên</label>
                  <input v-model="user.first_name" type="text" class="w-full bg-gray-100 border-none rounded p-3 text-sm outline-none focus:ring-1 focus:ring-gray-300" />
                </div>
              </div>

              <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                <div>
                  <label class="block text-sm mb-2 text-gray-700">Email</label>
                  <input v-model="user.email" type="email" class="w-full bg-gray-100 border-none rounded p-3 text-sm outline-none focus:ring-1 focus:ring-gray-300" />
                </div>
                <div>
                  <label class="block text-sm mb-2 text-gray-700">Địa chỉ</label>
                  <input v-model="user.address" type="text" class="w-full bg-gray-100 border-none rounded p-3 text-sm outline-none focus:ring-1 focus:ring-gray-300" />
                </div>
              </div>

              <div class="space-y-4 mb-8">
                <label class="block text-sm mb-2 text-gray-700">Mật khẩu</label>
                <input v-model="password.old" type="password" placeholder="mật khẩu cũ" class="w-full bg-gray-100 border-none rounded p-3 text-sm outline-none focus:ring-1 focus:ring-gray-300 placeholder-gray-400" />
                
                <input v-model="password.new" type="password" placeholder="mật khẩu mới" class="w-full bg-gray-100 border-none rounded p-3 text-sm outline-none focus:ring-1 focus:ring-gray-300 placeholder-gray-400" />
                
                <input v-model="password.confirm" type="password" placeholder="nhập lại mật khẩu mới" class="w-full bg-gray-100 border-none rounded p-3 text-sm outline-none focus:ring-1 focus:ring-gray-300 placeholder-gray-400" />
              </div>

              <div class="flex justify-end items-center gap-6 mt-10">
                <button type="button" class="text-sm text-gray-600 hover:text-black hover:underline">
                  Thoát
                </button>
                <button type="submit" class="bg-blue-600 text-white px-10 py-3 rounded text-sm hover:bg-blue-700 transition duration-300">
                  Lưu
                </button>
              </div>
              
            </form>
          </div>
        </main>

      </div>
    </div>
  </div>
</template>