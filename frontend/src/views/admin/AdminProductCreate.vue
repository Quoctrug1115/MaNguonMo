<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

const router = useRouter()
const isLoading = ref(false)
const isUploading = ref(false)
const uploadedGallery = ref([]) // Mảng chứa ảnh đã upload

const product = ref({
  category_id: null,
  name: '',
  description: '',
  price: 0,
  original_price: null,
  discount_percent: null,
  stock_quantity: 0,
  image_url: '',
  is_new: true,
  rating: null,
  reviews_count: null
})

const variants = ref([
  { color_name: '', color_hex: '#000000', stock: 0, image_url: '' }
])

const addVariant = () => { variants.value.push({ color_name: '', color_hex: '#ffffff', stock: 0, image_url: '' }) }
const removeVariant = (index) => { if (variants.value.length > 1) variants.value.splice(index, 1); else alert('Phải có ít nhất 1 màu!') }

// ==========================================
// HÀM XỬ LÝ UPLOAD NHIỀU ẢNH
// ==========================================
const handleFileUpload = async (event) => {
  const files = event.target.files
  if (!files || files.length === 0) return

  const formData = new FormData()
  for (let i = 0; i < files.length; i++) {
    formData.append('images', files[i]) // 'images' là tên field gửi lên Rust
  }

  try {
    isUploading.value = true
    const res = await axios.post('http://localhost:3000/api/admin/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    })

    const newUrls = res.data.urls
    uploadedGallery.value.push(...newUrls)

    // TÍNH NĂNG THÔNG MINH: Tự động gán link ảnh vào Form
    if (!product.value.image_url && newUrls.length > 0) {
      product.value.image_url = newUrls[0] // Ảnh 1 làm ảnh chính
    }
    
    // Gán các ảnh tiếp theo cho các biến thể (nếu biến thể chưa có ảnh)
    let urlIndex = 1;
    for (let i = 0; i < variants.value.length; i++) {
      if (!variants.value[i].image_url && urlIndex < newUrls.length) {
        variants.value[i].image_url = newUrls[urlIndex]
        urlIndex++
      }
    }

    alert(`✅ Đã tải lên ${newUrls.length} ảnh thành công!`)
  } catch (error) {
    console.error("Lỗi upload:", error)
    alert('Upload ảnh thất bại!')
  } finally {
    isUploading.value = false
    event.target.value = '' // Reset input
  }
}

// Hàm Submit
const submitProduct = async () => {
  try {
    isLoading.value = true
    const payload = {
      ...product.value,
      discount_percent: product.value.original_price ? Math.round((1 - product.value.price / product.value.original_price) * 100) : null,
      variants: variants.value
    }
    await axios.post('http://localhost:3000/api/admin/products', payload)
    alert('🎉 Thêm sản phẩm thành công!')
    router.push('/admin/manage-products')
  } catch (error) {
    alert('Thêm thất bại. Vui lòng kiểm tra lại!')
  } finally {
    isLoading.value = false
  }
}
</script>

<template>
  <div class="p-8 max-w-7xl mx-auto">
    <div class="flex justify-between items-center mb-8">
      <div>
        <button @click="router.push('/admin/manage-products')" class="text-sm text-gray-500 hover:text-blue-600 mb-2 flex items-center gap-1 transition-colors">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"/></svg> Quay lại
        </button>
        <h2 class="text-[28px] font-bold text-gray-800 tracking-wide">Thêm Sản Phẩm Mới</h2>
      </div>
      
      <div class="flex gap-3">
        <button @click="router.push('/admin/manage-products')" class="px-6 py-2.5 rounded-xl text-gray-600 font-bold bg-white border border-gray-200 hover:bg-gray-50 transition-colors">Hủy</button>
        <button @click="submitProduct" :disabled="isLoading" class="px-6 py-2.5 rounded-xl text-white font-bold bg-blue-600 shadow-md hover:bg-blue-700 transition-colors flex items-center gap-2">
          Lưu Sản Phẩm
        </button>
      </div>
    </div>

    <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-100 mb-8">
      <div class="flex justify-between items-end mb-4">
        <div>
          <h3 class="text-lg font-bold text-gray-800">Thư Viện Ảnh (Tải lên nhiều ảnh)</h3>
          <p class="text-sm text-gray-500 mt-1">Chọn tất cả ảnh của sản phẩm. Hệ thống sẽ tự động phân bổ vào Ảnh chính và Biến thể.</p>
        </div>
        
        <div class="relative overflow-hidden inline-block">
          <button :disabled="isUploading" class="bg-indigo-50 text-indigo-600 border border-indigo-200 px-5 py-2.5 rounded-lg font-bold text-sm hover:bg-indigo-100 transition-colors flex items-center gap-2 disabled:opacity-50">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"/></svg>
            {{ isUploading ? 'Đang tải lên...' : 'Chọn Ảnh Tải Lên' }}
          </button>
          <input @change="handleFileUpload" type="file" multiple accept="image/*" class="absolute top-0 left-0 w-full h-full opacity-0 cursor-pointer" />
        </div>
      </div>

      <div v-if="uploadedGallery.length > 0" class="flex gap-4 overflow-x-auto py-2">
        <div v-for="(url, idx) in uploadedGallery" :key="idx" class="relative group w-24 h-24 flex-shrink-0 rounded-xl border border-gray-200 overflow-hidden shadow-sm">
          <img :src="url" class="w-full h-full object-cover" />
          <button @click="navigator.clipboard.writeText(url); alert('Đã copy link!')" class="absolute inset-0 bg-black/50 text-white text-xs font-bold opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity cursor-pointer">
            Copy Link
          </button>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-3 gap-8">
      <div class="col-span-2 space-y-6">
        <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h3 class="text-lg font-bold text-gray-800 mb-5">Thông Tin Cơ Bản</h3>
          
          <div class="space-y-5">
            <div class="grid grid-cols-2 gap-5">
              <div>
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Tên sản phẩm <span class="text-red-500">*</span></label>
                <input v-model="product.name" type="text" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
              </div>
              <div>
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Category ID (UUID)</label>
                <input v-model="product.category_id" type="text" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
              </div>
            </div>

            <div class="grid grid-cols-3 gap-5">
              <div>
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Giá bán <span class="text-red-500">*</span></label>
                <input v-model.number="product.price" type="number" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
              </div>
              <div>
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Giá gốc</label>
                <input v-model.number="product.original_price" type="number" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
              </div>
              <div>
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Tổng Tồn Kho</label>
                <input v-model.number="product.stock_quantity" type="number" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
              </div>
            </div>

            <div class="grid grid-cols-4 gap-5 items-end">
              <div class="col-span-3">
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Link Ảnh Chính (Auto-fill)</label>
                <input v-model="product.image_url" type="text" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-blue-50/50 text-blue-700 outline-none focus:border-blue-500">
              </div>
              <div class="col-span-1 flex items-center h-10 px-2">
                <label class="flex items-center gap-2 cursor-pointer">
                  <input v-model="product.is_new" type="checkbox" class="w-5 h-5 text-blue-600 rounded border-gray-300">
                  <span class="text-sm font-bold text-gray-700">SP Mới (New)</span>
                </label>
              </div>
            </div>

            <div>
              <label class="block text-[13px] font-semibold text-gray-700 mb-1">Mô tả chi tiết</label>
              <textarea v-model="product.description" rows="3" class="w-full px-4 py-3 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500"></textarea>
            </div>
          </div>
        </div>
      </div>

      <div class="col-span-1">
        <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <div class="flex justify-between items-center mb-5">
            <h3 class="text-lg font-bold text-gray-800">Biến Thể Màu Sắc</h3>
            <button @click="addVariant" class="text-[13px] font-bold text-blue-600 hover:text-blue-800 bg-blue-50 px-3 py-1.5 rounded-lg">+ Thêm Màu</button>
          </div>

          <div class="space-y-4 max-h-[500px] overflow-y-auto pr-2">
            <div v-for="(variant, index) in variants" :key="index" class="p-4 rounded-xl border border-gray-100 bg-gray-50 relative">
              <button @click="removeVariant(index)" class="absolute top-2 right-2 text-gray-400 hover:text-red-500 p-1">X</button>
              <div class="space-y-3 mt-2">
                <div class="flex gap-3">
                  <div class="flex-grow">
                    <label class="block text-[12px] font-semibold text-gray-600 mb-1">Tên Màu</label>
                    <input v-model="variant.color_name" type="text" class="w-full px-3 py-2 rounded-lg border border-gray-200 text-sm outline-none">
                  </div>
                  <div class="w-12">
                    <label class="block text-[12px] font-semibold text-gray-600 mb-1">Mã</label>
                    <input v-model="variant.color_hex" type="color" class="w-full h-[36px] rounded-lg border border-gray-200 cursor-pointer p-0.5">
                  </div>
                </div>
                
                <div class="flex gap-3">
                  <div class="w-1/3">
                    <label class="block text-[12px] font-semibold text-gray-600 mb-1">Tồn kho</label>
                    <input v-model.number="variant.stock" type="number" class="w-full px-3 py-2 rounded-lg border border-gray-200 text-sm outline-none">
                  </div>
                  <div class="w-2/3">
                    <label class="block text-[12px] font-semibold text-gray-600 mb-1">Link Ảnh (Auto-fill)</label>
                    <input v-model="variant.image_url" type="text" class="w-full px-3 py-2 rounded-lg border border-gray-200 bg-blue-50/50 text-blue-700 text-sm outline-none">
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>