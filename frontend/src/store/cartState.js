import { ref } from 'vue'
import axios from 'axios'

// Biến lưu số lượng giỏ hàng dùng chung cho toàn web
export const cartCount = ref(0)

// Hàm gọi API để đếm lại số lượng
export const fetchCartCount = async () => {
  const userStr = localStorage.getItem('user')
  if (!userStr) {
    cartCount.value = 0
    return
  }
  
  try {
    const user = JSON.parse(userStr)
    const res = await axios.get(`http://localhost:3000/api/cart/${user.id}`)
    
    const items = res.data.data
    // Cộng dồn toàn bộ số lượng (quantity) của các món đồ trong giỏ
    cartCount.value = items.reduce((total, item) => total + item.quantity, 0)
  } catch (error) {
    console.error("Lỗi đếm số lượng giỏ hàng:", error)
  }
}