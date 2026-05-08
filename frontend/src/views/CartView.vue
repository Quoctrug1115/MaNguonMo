<script setup>
import { ref, computed, onMounted } from 'vue'
import axios from 'axios'
import { useRouter } from 'vue-router'

const router = useRouter()
const cartItems = ref([])


const fetchCart = async () => {
  const userStr = localStorage.getItem('user')
  
  if (!userStr) {
    // Nếu chưa đăng nhập thì đẩy về trang login
    router.push('/login')
    return
  }

  const user = JSON.parse(userStr)

  try {
    // Gọi API của Rust
    const res = await axios.get(`http://localhost:3000/api/cart/${user.id}`)
    
    // Gán dữ liệu thật vào biến. Nhớ cấu trúc res.data.data nhé!
    cartItems.value = res.data.data
  } catch (error) {
    console.error("Lỗi lấy giỏ hàng:", error)
  }
}

onMounted(() => {
  fetchCart()
})

const increaseQty = (item) => {
  updateQuantity(item, item.quantity + 1);
}

const decreaseQty = (item) => {
  if (item.quantity > 1) {
    updateQuantity(item, item.quantity - 1);
  }
}

const removeItem = async (cart_item_id) => {
  if (!confirm("Bạn có chắc chắn muốn xóa sản phẩm này?")) return;

  try {
    await axios.delete(`http://localhost:3000/api/cart/item/${cart_item_id}`);
    // Xóa xong thì lọc mảng cục bộ để biến mất khỏi màn hình ngay
    cartItems.value = cartItems.value.filter(i => i.cart_item_id !== cart_item_id);
  } catch (error) {
    console.error("Lỗi xóa sản phẩm:", error);
  }
}

// Tính tổng tiền dựa trên dữ liệu thật
const subtotal = computed(() => {
  return cartItems.value.reduce((total, item) => total + (item.price * item.quantity), 0)
})

// Hàm format tiền tệ
const formatPrice = (price) => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(price || 0)
}

const updateQuantity = async (item, newQty) => {
  if (newQty < 1) return; // Không cho giảm xuống dưới 1

  try {
    await axios.put(`http://localhost:3000/api/cart/item/${item.cart_item_id}`, {
      quantity: newQty
    });
    // Nếu API thành công, cập nhật số lượng hiển thị trên màn hình
    item.quantity = newQty;
  } catch (error) {
    console.error("Lỗi cập nhật số lượng:", error);
    alert("Không thể cập nhật số lượng!");
  }
};


</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20 text-gray-800">
    <nav class="text-sm text-gray-500 mb-10">
      <router-link to="/" class="hover:text-black">Trang chủ</router-link>
      <span class="mx-2">/</span>
      <span class="text-black font-medium">Cart</span>
    </nav>

    <div class="mb-8">
      <div class="grid grid-cols-12 gap-4 py-4 shadow-sm bg-white rounded-md mb-6 font-medium text-center md:text-left px-6">
        <div class="col-span-5 text-left">Sản phẩm</div>
        <div class="col-span-2">Giá</div>
        <div class="col-span-3">Số Lượng</div>
        <div class="col-span-2 text-right">Thành tiền</div>
      </div>

      <div v-for="item in cartItems" :key="item.cart_item_id" 
        class="grid grid-cols-12 gap-4 items-center py-6 shadow-sm bg-white rounded-md mb-4 px-6 relative group">
        <button @click="removeItem(item.cart_item_id)" class="absolute left-2 top-1/2 -translate-y-1/2 text-red-500 opacity-0 group-hover:opacity-100 transition-opacity p-2">
          ✖
        </button>

        <div class="col-span-5 flex items-center gap-4 pl-4">
          <img :src="item.image_url" class="w-12 h-12 object-cover rounded" alt="product" />
          <span class="font-medium truncate" :title="item.product_name">{{ item.product_name }}</span>
        </div>
        
        <div class="col-span-2 text-center md:text-left text-red-500 font-medium">{{ formatPrice(item.price) }}</div>
        
        <div class="col-span-3 flex justify-center md:justify-start">
          <div class="flex items-center border border-gray-300 rounded w-20 md:w-24 h-10">
            <span class="w-full text-center">{{ item.quantity }}</span>
            <div class="flex flex-col border-l border-gray-300 w-8 h-full">
              <button @click="increaseQty(item)" class="h-1/2 border-b border-gray-300 hover:bg-gray-100 flex items-center justify-center text-xs">▲</button>
              <button @click="decreaseQty(item)" class="h-1/2 hover:bg-gray-100 flex items-center justify-center text-xs">▼</button>
            </div>
          </div>
        </div>
        
        <div class="col-span-2 text-right font-medium text-red-500">{{ formatPrice(item.price * item.quantity) }}</div>
      </div>
    </div>

    <div class="flex justify-between items-center mb-16">
      <router-link to="/" class="px-8 py-3 border border-gray-400 rounded hover:bg-gray-50 font-medium transition">
        Trở về
      </router-link>
      <button class="px-8 py-3 border border-gray-400 rounded hover:bg-gray-50 font-medium transition">
        Update Cart
      </button>
    </div>

    <div class="flex flex-col md:flex-row justify-between gap-10 items-start">
      <div class="flex gap-4 w-full md:w-1/2">
        <input type="text" placeholder="Mã giảm giá" class="border border-black rounded px-4 py-3 flex-grow outline-none focus:ring-1 focus:ring-blue-500" />
        <button class="bg-blue-600 text-white px-8 py-3 rounded font-medium hover:bg-blue-700 transition whitespace-nowrap">
          Thêm
        </button>
      </div>

      <div class="border border-black rounded-md p-6 w-full md:w-[400px]">
        <h2 class="text-xl font-medium mb-6">Tổng tiền</h2>
        
        <div class="flex justify-between border-b border-gray-200 pb-4 mb-4">
          <span class="text-gray-600">Đơn hàng:</span>
          <span class="font-medium">{{ formatPrice(subtotal) }}</span>
        </div>
        
        <div class="flex justify-between border-b border-gray-200 pb-4 mb-4">
          <span class="text-gray-600">Phí vận chuyển:</span>
          <span class="font-medium">Free</span>
        </div>
        
        <div class="flex justify-between mb-6">
          <span class="text-gray-800 font-medium">Thành tiền:</span>
          <span class="font-bold text-lg">{{ formatPrice(subtotal) }}</span>
        </div>

        <router-link to="/checkout" class="block w-full text-center bg-blue-600 text-white py-3 rounded font-medium hover:bg-blue-700 transition">
          Đi Đến Thanh Toán
        </router-link>
      </div>
    </div>
  </div>
</template>