import { ref } from 'vue'
import axios from 'axios'

export const cartCount = ref(0)

export const fetchCartCount = async () => {
  const token = localStorage.getItem('token')
  
  // Nếu không có thẻ thông hành thì mặc định giỏ hàng = 0 và thoát luôn
  if (!token) {
    cartCount.value = 0
    return
  }
  
  try {
    // Gọi API chuẩn không cần chèn ID vào URL
    const res = await axios.get('http://localhost:3000/api/cart', {
      headers: { 'Authorization': `Bearer ${token}` }
    })
    
    const items = res.data.data
    cartCount.value = items.reduce((total, item) => total + item.quantity, 0)
  } catch (error) {
    console.error("Lỗi đếm số lượng giỏ hàng:", error)
  }
}