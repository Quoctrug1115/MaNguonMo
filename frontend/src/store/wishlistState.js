import { ref } from 'vue'
import axios from 'axios'

// 1. Biến lưu tổng số lượng món đồ yêu thích (để hiện trên Header)
export const wishlistCount = ref(0)

// 2. Biến chứa danh sách ID các sản phẩm đã thả tim (để bôi đỏ icon)
export const likedProductIds = ref([])

// 3. Hàm gọi API lấy dữ liệu và cập nhật 2 biến trên
export const fetchWishlist = async () => {
  const userStr = localStorage.getItem('user')
  
  if (!userStr) {
    wishlistCount.value = 0
    likedProductIds.value = []
    return
  }

  try {
    const user = JSON.parse(userStr)
    const res = await axios.get(`http://localhost:3000/api/wishlist/${user.id}`)
    
    const items = res.data.data
    
    // Cập nhật số lượng
    wishlistCount.value = items.length
    
    // Cập nhật danh sách ID (chỉ bóc lấy cái product_id nhét vào mảng)
    likedProductIds.value = items.map(item => item.product_id)
    
  } catch (error) {
    console.error("Lỗi đồng bộ danh sách yêu thích:", error)
  }
}