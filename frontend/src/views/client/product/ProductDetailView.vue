<script setup>
import { ref, onMounted, computed, watch } from 'vue'
import { useRoute } from 'vue-router'
import ProductCard from '@/components/common/ProductCard.vue'
import { fetchCartCount } from '@/store/cartState'
import axios from 'axios'

const route = useRoute()
const isLoading = ref(true)
const quantity = ref(1)
const activeTab = ref('description')

// Khai báo các biến lưu dữ liệu thật (đã gom lại cho gọn)
const productDetail = ref({
  product: null,
  specifications: [],
  reviews: []
})

const fetchProductDetail = async () => {
  try {
    const response = await fetch(`http://localhost:3000/api/products/${route.params.id}`)
    const result = await response.json()
    if (response.ok) {
      // Gán dữ liệu thật vào biến
      productDetail.value = result.data
    }
  } catch (error) {
    console.error('Lỗi lấy chi tiết sản phẩm:', error)
  } finally {
    isLoading.value = false
  }
}

// Các computed và watch giữ nguyên (chỉ sửa lại đường dẫn trỏ tới dữ liệu)
const product = computed(() => productDetail.value.product)
const specifications = computed(() => productDetail.value.specifications)
const reviews = computed(() => productDetail.value.reviews)

const formattedPrice = computed(() => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(product.value?.price || 0)
})
const formattedOriginalPrice = computed(() => {
  return new Intl.NumberFormat('vi-VN', { style: 'currency', currency: 'VND' }).format(product.value?.original_price || 0)
})

// Các hàm Slider giữ nguyên (sửa lại document.getElementById)
const slideLeft = () => { document.getElementById('related-slider')?.scrollBy({ left: -320, behavior: 'smooth' }) }
const slideRight = () => { document.getElementById('related-slider')?.scrollBy({ left: 320, behavior: 'smooth' }) }

const relatedProducts = ref([])
const fetchRelatedProducts = async () => {
  try {
    const response = await fetch(`http://localhost:3000/api/products?limit=12`)
    const result = await response.json()
    if (response.ok) {
      relatedProducts.value = result.data.filter(item => item.id !== route.params.id)
    }
  } catch (error) { console.error('Lỗi liên quan:', error) }
}

onMounted(() => { fetchProductDetail(); fetchRelatedProducts() })
watch(() => route.params.id, (newId) => {
  if (newId) { isLoading.value = true; activeTab.value = 'description'; fetchProductDetail(); fetchRelatedProducts(); window.scrollTo({ top: 0, behavior: 'smooth' }) }
})

// Định nghĩa hàm formatDate để hiển thị ngày đánh giá đẹp hơn
const formatDate = (dateStr) => {
  return new Date(dateStr).toLocaleDateString('vi-VN', { year: 'numeric', month: '2-digit', day: '2-digit' })
}


const handleAddToCart = async () => {
  // 1. Kiểm tra đăng nhập
  const userStr = localStorage.getItem('user')
  const token = localStorage.getItem('token')

  if (!userStr || !token) {
    alert('Vui lòng đăng nhập để thêm sản phẩm vào giỏ hàng!')
    router.push('/login')
    return
  }

  const user = JSON.parse(userStr)

  // 2. Gọi API thêm vào giỏ
  try {
    const response = await axios.post('http://localhost:3000/api/cart', {
      user_id: user.id,
      product_id: product.value.id, // Lấy ID của sản phẩm đang xem
      quantity: quantity.value || 1 // Lấy số lượng người dùng chọn (mặc định là 1)
    })

    if (response.status === 200) {
      alert(`🛒 Đã thêm ${quantity.value} [${product.value.name}] vào giỏ hàng!`)
      
      // 3. Cập nhật lại con số màu đỏ trên Header
      fetchCartCount() 
    }
  } catch (error) {
    console.error('Lỗi khi thêm vào giỏ hàng:', error)
    alert('Có lỗi xảy ra, không thể thêm vào giỏ hàng lúc này.')
  }
}

</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-12 mb-20">
    <!-- Breadcrumb -->
    <nav class="text-sm text-gray-500 mb-8 flex items-center gap-2">
      <router-link to="/" class="hover:text-primary">Trang chủ</router-link>
      <span>/</span>
      <router-link to="/products" class="hover:text-primary">Sản phẩm</router-link>
      <span>/</span>
      <span class="text-gray-900 font-medium truncate">{{ product?.name }}</span>
    </nav>

    <div v-if="isLoading" class="flex justify-center py-32">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-primary"></div>
    </div>

    <div v-else-if="product">
      <!-- =========================================
           PHẦN 1: THÔNG TIN CƠ BẢN (TRÊN CÙNG)
           ========================================= -->
      <div class="grid grid-cols-1 md:grid-cols-12 gap-10 mb-16">

        <!-- Cột trái: Hình ảnh (Chiếm 5 cột) -->
        <div class="md:col-span-5 flex gap-4">
          <!-- Thumbnail nhỏ (Mock) -->
          <div class="flex flex-col gap-3 w-20 hidden sm:flex">
            <div v-for="i in 4" :key="i" class="border rounded-md p-1 cursor-pointer hover:border-primary transition" :class="i===1 ? 'border-primary' : 'border-gray-200'">
              <img :src="product.image_url" class="w-full h-16 object-contain" />
            </div>
          </div>
          <!-- Ảnh to -->
          <div class="flex-1 bg-gray-50 rounded-lg p-6 flex items-center justify-center border border-gray-100">
            <img :src="product.image_url" :alt="product.name" class="w-full max-h-[400px] object-contain" />
          </div>
        </div>

        <!-- Cột phải: Thông tin & Mua hàng (Chiếm 7 cột) -->
        <div class="md:col-span-7 flex flex-col">
          <h1 class="text-3xl font-bold text-gray-900 mb-3">{{ product.name }}</h1>

          <!-- Rating -->
          <div class="flex items-center gap-3 mb-4 text-sm">
            <div class="flex text-yellow-400">
              <svg v-for="i in 5" :key="i" class="w-4 h-4 fill-current" viewBox="0 0 24 24"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg>
            </div>
            <span class="text-gray-500">({{ product.reviews_count }} Đánh giá)</span>
            <span class="text-gray-300">|</span>
            <span class="text-green-500 font-medium">Còn hàng ({{ product.stock_quantity }})</span>
          </div>

          <!-- Giá -->
          <div class="mb-6">
            <div class="flex items-end gap-3 mb-2">
              <span class="text-3xl font-bold text-red-500">{{ formattedPrice }}</span>
              <span v-if="product.original_price > product.price" class="text-lg text-gray-400 line-through mb-1">{{ formattedOriginalPrice }}</span>
            </div>
          </div>

          <hr class="border-gray-200 mb-6" />

          <!-- Nút mua hàng -->
          <div class="flex items-center gap-4 mb-8">
            <div class="flex items-center border border-gray-300 rounded-md h-12">
              <button @click="quantity > 1 && quantity--" class="px-4 text-lg hover:bg-gray-50">-</button>
              <input v-model="quantity" type="number" class="w-14 text-center border-none focus:ring-0 font-medium" />
              <button @click="quantity++" class="px-4 text-lg hover:bg-gray-50">+</button>
            </div>
            <button @click="handleAddToCart" class="flex-grow bg-blue-600 text-white h-12 rounded-md font-bold hover:bg-blue-700 transition">
              Thêm vào giỏ hàng
            </button>
            <button class="h-12 w-12 border border-gray-300 rounded-md flex items-center justify-center hover:bg-gray-50 text-gray-500">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
            </button>
          </div>

          <!-- Thông tin vận chuyển nhanh -->
          <div class="border border-gray-200 rounded-md flex flex-col">
            <div class="flex items-center gap-4 p-4 border-b border-gray-200">
              <svg class="w-8 h-8 text-gray-700" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4"/></svg>
              <div>
                <p class="font-medium text-sm">Giao hàng miễn phí</p>
                <a href="#" class="text-xs text-gray-500 underline">Nhập mã bưu điện để kiểm tra</a>
              </div>
            </div>
            <div class="flex items-center gap-4 p-4">
              <svg class="w-8 h-8 text-gray-700" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg>
              <div>
                <p class="font-medium text-sm">Đổi trả trong 30 ngày</p>
                <p class="text-xs text-gray-500">Miễn phí đổi trả. Áp dụng điều kiện.</p>
              </div>
            </div>
          </div>

        </div>
      </div>

      <!-- =========================================
           PHẦN 2: 4 TABS THÔNG TIN CHI TIẾT
           ========================================= -->
      <div class="border border-gray-200 rounded-lg bg-white overflow-hidden shadow-sm">

        <!-- Thanh Navigation Tabs -->
        <ul class="flex flex-wrap border-b border-gray-200 bg-gray-50/50">
          <li
              @click="activeTab = 'description'"
              class="px-6 py-4 font-medium text-sm cursor-pointer border-b-2 transition-colors flex items-center gap-2"
              :class="activeTab === 'description' ? 'border-pink-500 text-pink-600 bg-white' : 'border-transparent text-gray-600 hover:text-pink-500'"
          >
            📄 Mô tả sản phẩm
          </li>
          <li
              @click="activeTab = 'specs'"
              class="px-6 py-4 font-medium text-sm cursor-pointer border-b-2 transition-colors flex items-center gap-2"
              :class="activeTab === 'specs' ? 'border-pink-500 text-pink-600 bg-white' : 'border-transparent text-gray-600 hover:text-pink-500'"
          >
            📊 Thông số kỹ thuật
          </li>
          <li
              @click="activeTab = 'reviews'"
              class="px-6 py-4 font-medium text-sm cursor-pointer border-b-2 transition-colors flex items-center gap-2"
              :class="activeTab === 'reviews' ? 'border-pink-500 text-pink-600 bg-white' : 'border-transparent text-gray-600 hover:text-pink-500'"
          >
            ⭐ Đánh giá ({{ product.reviews_count }})
          </li>
          <li
              @click="activeTab = 'shipping'"
              class="px-6 py-4 font-medium text-sm cursor-pointer border-b-2 transition-colors flex items-center gap-2"
              :class="activeTab === 'shipping' ? 'border-pink-500 text-pink-600 bg-white' : 'border-transparent text-gray-600 hover:text-pink-500'"
          >
            🚚 Vận chuyển & Đổi trả
          </li>
        </ul>

        <!-- Nội dung Tabs -->
        <div class="p-6 md:p-8">

          <!-- Tab 1: Mô tả -->
          <div v-show="activeTab === 'description'" class="grid grid-cols-1 md:grid-cols-3 gap-10">
            <div class="md:col-span-2">
              <p class="text-gray-700 leading-relaxed mb-6 whitespace-pre-line">{{ product.description }}</p>

              <div class="bg-orange-50 rounded-lg p-6">
                <h3 class="font-bold text-orange-800 mb-4 flex items-center gap-2">✨ Điểm nổi bật</h3>
                <ul class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm text-gray-700">
                  <li class="flex items-center gap-2"><span class="text-pink-500">✓</span> Hàng chính hãng 100%</li>
                  <li class="flex items-center gap-2"><span class="text-pink-500">✓</span> Xuất xứ rõ ràng</li>
                  <li class="flex items-center gap-2"><span class="text-pink-500">✓</span> Giao hàng toàn quốc</li>
                  <li class="flex items-center gap-2"><span class="text-pink-500">✓</span> Tư vấn miễn phí 24/7</li>
                </ul>
              </div>
            </div>

            <div class="md:col-span-1 flex flex-col gap-6">
              <div class="border border-pink-100 rounded-lg p-5">
                <h3 class="font-bold text-pink-600 mb-4 flex items-center gap-2">🛡️ Cam kết cửa hàng</h3>
                <div class="flex flex-col gap-4 text-sm">
                  <div class="flex gap-3">
                    <span class="text-green-500">✅</span>
                    <div><p class="font-semibold">Chính hãng 100%</p><p class="text-gray-500 text-xs">Cam kết hàng thật, đền bù gấp đôi nếu phát hiện giả.</p></div>
                  </div>
                  <div class="flex gap-3">
                    <span class="text-blue-500">🔄</span>
                    <div><p class="font-semibold">Đổi trả trong 7 ngày</p><p class="text-gray-500 text-xs">Đổi trả miễn phí nếu sản phẩm lỗi từ NSX.</p></div>
                  </div>
                  <div class="flex gap-3">
                    <span class="text-orange-500">🚚</span>
                    <div><p class="font-semibold">Giao hàng toàn quốc</p><p class="text-gray-500 text-xs">Miễn phí cho đơn hàng từ 500.000đ.</p></div>
                  </div>
                </div>
              </div>
              <div class="bg-gray-50 rounded-lg p-5 text-center border border-gray-100">
                <div class="text-pink-600 mb-2">📞</div>
                <p class="font-bold text-gray-800 mb-1">Hotline tư vấn</p>
                <p class="text-2xl font-bold text-pink-600 mb-1">1800 6996</p>
                <p class="text-xs text-gray-500">Miễn phí • 8:00 - 22:00 hàng ngày</p>
              </div>
            </div>
          </div>

          <!-- Tab 2: Thông số kỹ thuật (DỮ LIỆU THẬT) -->
          <div v-show="activeTab === 'specs'">
            <h3 class="font-bold text-gray-900 mb-6 flex items-center gap-2">📄 Thông số chi tiết</h3>

            <div v-if="specifications.length === 0" class="text-center py-10 text-gray-500 text-sm bg-gray-50 rounded-md">
              Thông số kỹ thuật đang được cập nhật...
            </div>

            <div v-else class="border border-gray-200 rounded-md overflow-hidden shadow-sm">
              <table class="w-full text-sm text-left">
                <tbody>
                <!-- Lặp qua từng thông số kỹ thuật lấy từ DB -->
                <tr v-for="(spec, index) in specifications" :key="index" class="border-b border-gray-200" :class="index % 2 === 0 ? 'bg-gray-50/70' : 'bg-white'">
                  <th class="py-4 px-6 font-semibold text-gray-700 w-1/3">{{ spec.spec_key }}</th>
                  <td class="py-4 px-6 text-gray-900 leading-relaxed">{{ spec.spec_value }}</td>
                </tr>
                </tbody>
              </table>
            </div>
          </div>

          <!-- Tab 3: Đánh giá -->
          <div v-show="activeTab === 'reviews'">
            <div class="flex flex-col md:flex-row gap-10 mb-10 border-b pb-10">
              <!-- Cột điểm số -->
              <div class="md:w-1/3 flex flex-col items-center justify-center text-center">
                <div class="text-5xl font-bold text-gray-900 mb-2">{{ product.rating }}</div>
                <div class="flex text-yellow-400 mb-2">
                  <svg v-for="i in 5" :key="i" class="w-5 h-5 fill-current" viewBox="0 0 24 24"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg>
                </div>
                <p class="text-gray-500 text-sm">{{ product.reviews_count }} đánh giá</p>
              </div>

              <!-- Cột Progress bar (Mock visual) -->
              <div class="md:w-2/3 flex flex-col gap-2 justify-center">
                <div v-for="i in [5,4,3,2,1]" :key="i" class="flex items-center gap-3 text-sm text-gray-600">
                  <span>{{ i }} ⭐</span>
                  <div class="flex-1 bg-gray-200 h-2 rounded-full overflow-hidden">
                    <div class="bg-yellow-400 h-full" :style="{ width: i === 5 ? '80%' : i === 4 ? '15%' : '2%' }"></div>
                  </div>
                </div>
              </div>
            </div>

            <!-- List review (DỮ LIỆU THẬT) -->
            <div v-if="reviews.length === 0" class="text-center py-16 text-gray-500 text-sm border border-gray-100 rounded-lg mb-10">
              Chưa có đánh giá nào cho sản phẩm này. Hãy là người đầu tiên đánh giá!
            </div>

            <div v-else class="flex flex-col gap-6 mb-10">
              <!-- Lặp qua từng đánh giá lấy từ DB -->
              <div v-for="review in reviews" :key="review.id" class="border border-gray-100 bg-gray-50/40 rounded-lg p-6 shadow-sm">
                <div class="flex justify-between mb-4">
                  <div class="flex items-center gap-3">
                    <!-- Lấy chữ cái đầu làm Avatar -->
                    <div class="w-11 h-11 rounded-full bg-orange-400 text-white flex items-center justify-center font-bold text-lg shadow-inner">{{ review.user_name.charAt(0).toUpperCase() }}</div>
                    <div>
                      <p class="font-bold text-gray-900">{{ review.user_name }}</p>
                      <div class="flex text-yellow-400 mt-1">
                        <svg v-for="i in 5" :key="i" class="w-4 h-4 fill-current" :class="i <= review.rating ? 'text-yellow-400' : 'text-gray-200'" viewBox="0 0 24 24"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg>
                      </div>
                    </div>
                  </div>
                  <!-- Dùng hàm formatDate mới -->
                  <span class="text-xs text-gray-400">{{ formatDate(review.created_at) }}</span>
                </div>
                <p class="text-gray-700 text-sm mb-3 leading-relaxed">{{ review.content }}</p>
                <div v-if="review.is_verified" class="inline-flex items-center gap-1.5 bg-green-50 text-green-700 text-[11px] px-2.5 py-1 rounded-sm font-medium border border-green-100">
                  <span>✓</span> Đã mua hàng tại Cửa hàng
                </div>
              </div>
            </div>

            <!-- Viết đánh giá -->
            <div class="bg-gray-50 rounded-lg p-6">
              <h4 class="font-bold text-gray-800 mb-4">✍️ Viết đánh giá của bạn</h4>
              <div class="flex gap-2 text-gray-300 mb-4 cursor-pointer">
                <svg v-for="i in 5" :key="i" class="w-8 h-8 fill-current hover:text-yellow-400 transition" viewBox="0 0 24 24"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg>
              </div>
              <textarea placeholder="Chia sẻ trải nghiệm của bạn về sản phẩm này..." class="w-full border-gray-300 rounded-md shadow-sm focus:ring-pink-500 focus:border-pink-500 text-sm mb-4" rows="4"></textarea>
              <button class="bg-pink-500 text-white px-6 py-2 rounded-md font-bold hover:bg-pink-600 transition">Gửi đánh giá</button>
            </div>
          </div>

          <!-- Tab 4: Vận chuyển & Đổi trả -->
          <div v-show="activeTab === 'shipping'" class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="flex flex-col gap-4">
              <h4 class="font-bold text-gray-800 flex items-center gap-2 mb-2">🚚 Chính sách vận chuyển</h4>
              <div class="border border-gray-100 bg-gray-50 rounded-md p-4 flex gap-4">
                <div class="text-2xl">📦</div>
                <div>
                  <p class="font-bold text-sm text-gray-800">Giao hàng tiêu chuẩn</p>
                  <p class="text-xs text-gray-500 mt-1">2-4 ngày làm việc • Phí 30.000đ<br/>Miễn phí cho đơn từ 500.000đ</p>
                </div>
              </div>
              <div class="border border-gray-100 bg-gray-50 rounded-md p-4 flex gap-4">
                <div class="text-2xl text-orange-500">⚡</div>
                <div>
                  <p class="font-bold text-sm text-gray-800">Giao hàng nhanh (Nội thành)</p>
                  <p class="text-xs text-gray-500 mt-1">Trong ngày • Phí 50.000đ<br/>Áp dụng cho HCM, HN, Đà Nẵng</p>
                </div>
              </div>
            </div>

            <div class="flex flex-col gap-4">
              <h4 class="font-bold text-gray-800 flex items-center gap-2 mb-2">🔄 Chính sách đổi trả</h4>
              <div class="border border-green-100 bg-green-50 rounded-md p-4 flex gap-4">
                <div class="text-2xl text-green-500">✅</div>
                <div>
                  <p class="font-bold text-sm text-green-800">Đổi trả trong 7 ngày</p>
                  <p class="text-xs text-green-600 mt-1">Kể từ ngày nhận hàng<br/>Sản phẩm còn nguyên tem, hộp</p>
                </div>
              </div>
              <div class="border border-blue-100 bg-blue-50 rounded-md p-4 flex gap-4">
                <div class="text-2xl">💰</div>
                <div>
                  <p class="font-bold text-sm text-blue-800">Hoàn tiền 100%</p>
                  <p class="text-xs text-blue-600 mt-1">Nếu hàng bị lỗi do nhà sản xuất<br/>Hoàn tiền trong 24-48 giờ</p>
                </div>
              </div>
            </div>
          </div>

          <!-- =========================================
           PHẦN 3: SẢN PHẨM LIÊN QUAN (Có Slider)
           ========================================= -->
          <div class="mt-20">

            <!-- Phần Tiêu đề & 2 Nút điều hướng -->
            <div class="flex items-center justify-between mb-8">
              <div class="flex items-center gap-3">
                <div class="w-2 h-8 bg-blue-600 rounded-sm"></div>
                <h2 class="text-2xl font-bold text-gray-900">Sản phẩm liên quan</h2>
              </div>

              <!-- Cặp nút mũi tên -->
              <div class="flex gap-3">
                <button @click="slideLeft" class="w-11 h-11 rounded-full bg-gray-50 flex items-center justify-center text-gray-600 hover:bg-gray-200 hover:text-gray-900 transition shadow-sm border border-gray-100">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/></svg>
                </button>
                <button @click="slideRight" class="w-11 h-11 rounded-full bg-gray-50 flex items-center justify-center text-gray-600 hover:bg-gray-200 hover:text-gray-900 transition shadow-sm border border-gray-100">
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3"/></svg>
                </button>
              </div>
            </div>

            <!-- Thanh trượt chứa danh sách sản phẩm -->
            <div
                id="related-slider"
                class="flex gap-6 overflow-x-auto snap-x snap-mandatory pb-4 hide-scrollbar"
            >
              <!-- Mỗi thẻ sản phẩm sẽ chiếm 25% màn hình Desktop, 50% màn Mobile -->
              <div
                  v-for="item in relatedProducts"
                  :key="item.id"
                  class="min-w-[calc(50%-12px)] md:min-w-[calc(25%-18px)] snap-start flex-shrink-0"
              >
                <ProductCard :product="item" />
              </div>
            </div>

          </div>

        </div>
      </div>

    </div>
  </div>
</template>
<style scoped>
/* Ẩn thanh cuộn trên Chrome, Safari và Opera */
.hide-scrollbar::-webkit-scrollbar {
  display: none;
}

/* Ẩn thanh cuộn trên IE, Edge và Firefox */
.hide-scrollbar {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}
</style>