<script setup>
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import axios from 'axios'

const router = useRouter()
const route = useRoute()
const productId = route.params.id // Lấy ID từ URL
const isLoading = ref(false)
const isUploading = ref(false)
const categoriesList = ref([])
const uploadedGallery = ref([])

const product = ref({
  category_id: null, name: '', description: '', price: 0,
  original_price: null, discount_percent: null, stock_quantity: 0,
  image_url: '', is_new: true
})

const variants = ref([])
const specifications = ref([])

// 1. LẤY DỮ LIỆU CŨ KHI LOAD TRANG
const fetchOldData = async () => {
    console.log("Đang lấy dữ liệu cho sản phẩm ID:", productId);
  try {
    const res = await axios.get(`http://localhost:3000/api/admin/products/${productId}`)
    const oldData = res.data.data
    
    // Đổ dữ liệu vào Form
    product.value = oldData.product
    variants.value = oldData.variants
    specifications.value = oldData.specifications
    
    // Nếu chưa có mảng nào thì khởi tạo mảng trống để hiện giao diện
    if (variants.value.length === 0) addVariant()
    if (specifications.value.length === 0) addSpecification()
    
  } catch (error) {
    console.error("Lỗi khi lấy dữ liệu sản phẩm:", error)
    alert("Không tìm thấy sản phẩm này!")
    router.push('/admin/manage-products')
  }
}

const fetchCategories = async () => {
  const res = await axios.get('http://localhost:3000/api/categories')
  categoriesList.value = res.data
}

onMounted(() => {
  fetchCategories()
  fetchOldData()
})

// Các hàm bổ trợ (Giống trang Create)
const addVariant = () => variants.value.push({ color_name: '', color_hex: '#ffffff', stock: 0, image_url: '' })
const removeVariant = (index) => variants.value.splice(index, 1)
const addSpecification = () => specifications.value.push({ spec_name: '', spec_value: '' })
const removeSpecification = (index) => specifications.value.splice(index, 1)

// Hàm Submit Cập Nhật
const handleUpdate = async () => {
  try {
    isLoading.value = true
    const token = localStorage.getItem('token')
    
    const payload = {
      ...product.value,
      discount_percent: product.value.original_price ? Math.round((1 - product.value.price / product.value.original_price) * 100) : null,
      variants: variants.value,
      specifications: specifications.value.filter(s => s.spec_name.trim() !== '')
    }

    await axios.put(`http://localhost:3000/api/admin/products/${productId}`, payload, {
      headers: { 'Authorization': `Bearer ${token}` }
    })

    alert('🎉 Cập nhật sản phẩm thành công!')
    router.push('/admin/manage-products')
  } catch (error) {
    alert('Lỗi cập nhật. Vui lòng kiểm tra console.')
  } finally {
    isLoading.value = false
  }
}

// HÀM COPY LINK CHỐNG LỖI BẢO MẬT TRÌNH DUYỆT
const copyToClipboard = (text) => {
  if (navigator.clipboard && window.isSecureContext) {
    navigator.clipboard.writeText(text)
      .then(() => alert('✅ Đã copy link: ' + text))
      .catch(err => console.error('Lỗi copy:', err))
  } else {
    // Dùng mẹo tạo thẻ input ẩn để copy cho http://localhost
    const textArea = document.createElement("textarea");
    textArea.value = text;
    textArea.style.position = "absolute";
    textArea.style.left = "-999999px";
    document.body.prepend(textArea);
    textArea.select();
    try {
      document.execCommand('copy');
      alert('✅ Đã copy link: ' + text);
    } catch (error) {
      console.error('Lỗi copy fallback:', error);
    } finally {
      textArea.remove();
    }
  }
}

// Hàm Upload Ảnh
const handleFileUpload = async (event) => {
  const files = event.target.files
  if (!files || files.length === 0) return

  const formData = new FormData()
  for (let i = 0; i < files.length; i++) formData.append('images', files[i])

  try {
    isUploading.value = true
    const res = await axios.post('http://localhost:3000/api/admin/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    })

    const newUrls = res.data.urls
    uploadedGallery.value.push(...newUrls)

    if (!product.value.image_url && newUrls.length > 0) product.value.image_url = newUrls[0]
    
    let urlIndex = 1;
    for (let i = 0; i < variants.value.length; i++) {
      if (!variants.value[i].image_url && urlIndex < newUrls.length) {
        variants.value[i].image_url = newUrls[urlIndex]; urlIndex++;
      }
    }
  } catch (error) {
    alert('Upload ảnh thất bại!')
  } finally {
    isUploading.value = false; event.target.value = ''
  }
}

</script>

<template>
  <div class="p-8 max-w-7xl mx-auto">
    <div class="flex justify-between items-center mb-8">
      <div>
        <button @click="router.push('/admin/manage-products')" class="text-sm text-gray-500 hover:text-blue-600 mb-2 flex items-center gap-1 transition-colors">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/></svg> 
          Quay lại danh sách
        </button>
        <h2 class="text-[28px] font-bold text-gray-800 tracking-wide">Chỉnh Sửa Sản Phẩm</h2>
      </div>
      
      <div class="flex gap-3">
        <button @click="router.push('/admin/manage-products')" class="px-6 py-2.5 rounded-xl text-gray-600 font-bold bg-white border border-gray-200 hover:bg-gray-50 transition-colors">
          Hủy
        </button>
        <button @click="handleUpdate" :disabled="isLoading" class="px-6 py-2.5 rounded-xl text-white font-bold bg-blue-600 shadow-md hover:bg-blue-700 transition-colors flex items-center gap-2">
          <span v-if="isLoading">Đang lưu...</span>
          <span v-else>Lưu Thay Đổi</span>
        </button>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
      <div class="lg:col-span-2 space-y-8">
        
        <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h3 class="text-lg font-bold text-gray-800 mb-4 border-b pb-2">Thông Tin Cơ Bản</h3>
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-5 mb-5">
            <div>
              <label class="block text-[13px] font-semibold text-gray-700 mb-1">Tên sản phẩm <span class="text-red-500">*</span></label>
              <input v-model="product.name" type="text" placeholder="Nhập tên sản phẩm..." class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
            </div>
            
            <div>
              <label class="block text-[13px] font-semibold text-gray-700 mb-1">Danh mục</label>
              <select v-model="product.category_id" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500 cursor-pointer">
                <option :value="null">-- Vui lòng chọn danh mục --</option>
                <option v-for="cat in categoriesList" :key="cat.id" :value="cat.id">
                  {{ cat.name }}
                </option>
              </select>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-3 gap-5 mb-5">
            <div>
              <label class="block text-[13px] font-semibold text-gray-700 mb-1">Giá bán (VNĐ) <span class="text-red-500">*</span></label>
              <input v-model="product.price" type="number" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
            </div>
            <div>
              <label class="block text-[13px] font-semibold text-gray-700 mb-1">Giá gốc (VNĐ)</label>
              <input v-model="product.original_price" type="number" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
            </div>
            <div>
              <label class="block text-[13px] font-semibold text-gray-700 mb-1">Tồn kho chung</label>
              <input v-model="product.stock_quantity" type="number" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
            </div>
          </div>

          <div>
            <label class="block text-[13px] font-semibold text-gray-700 mb-1">Mô tả sản phẩm</label>
            <textarea v-model="product.description" rows="4" placeholder="Nhập mô tả chi tiết..." class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500"></textarea>
          </div>
        </div>

        <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <div class="flex justify-between items-center mb-4 border-b pb-2">
            <h3 class="text-lg font-bold text-gray-800">Thông Số Kỹ Thuật</h3>
            <button @click="addSpecification" class="text-sm font-semibold text-blue-600 hover:text-blue-800 flex items-center gap-1">
              + Thêm thông số
            </button>
          </div>
          
          <div v-for="(spec, index) in specifications" :key="index" class="flex gap-3 mb-3 items-start">
            <input v-model="spec.spec_name" type="text" placeholder="Tên thông số (VD: RAM)" class="w-1/3 px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
            <input v-model="spec.spec_value" type="text" placeholder="Giá trị (VD: 8GB)" class="flex-1 px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
            <button @click="removeSpecification(index)" class="p-2 text-red-500 hover:bg-red-50 rounded-xl transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/></svg>
            </button>
          </div>
        </div>

      </div>

      <div class="space-y-8">
        
        <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h3 class="text-lg font-bold text-gray-800 mb-4 border-b pb-2">Ảnh Sản Phẩm</h3>
          
          <div class="mb-5">
            <label class="block text-[13px] font-semibold text-gray-700 mb-1">Ảnh đại diện chính</label>
            <input type="file" @change="handleFileUpload" class="w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100 mb-2"/>
            <input v-model="product.image_url" type="text" placeholder="Hoặc dán Link ảnh vào đây..." class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500 text-sm">
            
            <div v-if="product.image_url" class="mt-3 relative rounded-xl overflow-hidden border border-gray-200 aspect-video flex justify-center bg-gray-50">
              <img :src="product.image_url" class="object-contain h-full w-full" alt="Preview">
            </div>
          </div>

          <div class="flex items-center gap-2 mt-4 pt-4 border-t">
            <input v-model="product.is_new" type="checkbox" id="is_new" class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500">
            <label for="is_new" class="text-sm font-semibold text-gray-700 cursor-pointer">Đánh dấu là Hàng Mới (New)</label>
          </div>
        </div>

        <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <div class="flex justify-between items-center mb-4 border-b pb-2">
            <h3 class="text-lg font-bold text-gray-800">Biến Thể (Màu)</h3>
            <button @click="addVariant" class="text-sm font-semibold text-blue-600 hover:text-blue-800 flex items-center gap-1">
              + Thêm
            </button>
          </div>

          <div v-for="(variant, index) in variants" :key="index" class="bg-gray-50 p-4 rounded-xl border border-gray-100 mb-3 relative">
            <button @click="removeVariant(index)" class="absolute top-2 right-2 text-gray-400 hover:text-red-500">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
            </button>
            
            <div class="grid grid-cols-2 gap-3 mb-3">
              <div>
                <label class="block text-[11px] font-semibold text-gray-500 mb-1">Tên màu</label>
                <input v-model="variant.color_name" type="text" class="w-full px-3 py-1.5 text-sm rounded-lg border border-gray-200 outline-none focus:border-blue-500">
              </div>
              <div>
                <label class="block text-[11px] font-semibold text-gray-500 mb-1">Mã màu (Hex)</label>
                <input v-model="variant.color_hex" type="color" class="w-full h-[34px] rounded-lg cursor-pointer">
              </div>
            </div>
            
            <div class="grid grid-cols-2 gap-3">
              <div>
                <label class="block text-[11px] font-semibold text-gray-500 mb-1">Tồn kho</label>
                <input v-model="variant.stock" type="number" class="w-full px-3 py-1.5 text-sm rounded-lg border border-gray-200 outline-none focus:border-blue-500">
              </div>
              <div>
                <label class="block text-[11px] font-semibold text-gray-500 mb-1">Link Ảnh</label>
                <input v-model="variant.image_url" type="text" class="w-full px-3 py-1.5 text-sm rounded-lg border border-gray-200 outline-none focus:border-blue-500">
              </div>
            </div>
          </div>
          
        </div>

      </div>
    </div>
  </div>
</template>