import { ref, computed } from 'vue' //
import axios from 'axios'

export const likedProductIds = ref([])

export const wishlistCount = computed(() => likedProductIds.value.length)

export const fetchWishlist = async () => {
  const token = localStorage.getItem('token')
  
  if (!token) {
    likedProductIds.value = []
    return
  }

  try {
    const response = await axios.get(`http://localhost:3000/api/wishlist`, {
      headers: { 'Authorization': `Bearer ${token}` }
    })
    
    likedProductIds.value = response.data.data.map(item => item.product_id)
  } catch (error) {
    console.error('Lỗi khi lấy danh sách yêu thích:', error)
  }
}