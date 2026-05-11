<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

const router = useRouter()
const isLoading = ref(false)

// 1. DỮ LIỆU KHỚP 100% VỚI STRUCT CreateProductRequest CỦA BẠN
const product = ref({
  category_id: null, // Backend cần UUID, tạm để null hoặc bạn gán ID thật vào đây
  name: '',
  description: '',
  price: 0,
  original_price: null,
  discount_percent: null,
  stock_quantity: 0, // Cột này bạn có trong struct
  image_url: '',
  is_new: true, // Mặc định là sản phẩm mới
  rating: null,
  reviews_count: null
})

// 2. Dữ liệu Biến thể màu sắc (Giữ nguyên)
const variants = ref([
  { color_name: '', color_hex: '#000000', stock: 0, image_url: '' }
])

const addVariant = () => {
  variants.value.push({ color_name: '', color_hex: '#ffffff', stock: 0, image_url: '' })
}
const removeVariant = (index) => {
  if (variants.value.length > 1) variants.value.splice(index, 1)
  else alert('Sản phẩm phải có ít nhất 1 màu sắc!')
}

// 3. Hàm Submit
const submitProduct = async () => {
  try {
    isLoading.value = true
    
    // Gom dữ liệu: Product Info + Variants
    const payload = {
      ...product.value,
      // Tính toán auto: Nếu có nhập giá gốc & giá bán, tự tính % giảm giá (nếu bạn muốn)
      discount_percent: product.value.original_price ? 
        Math.round((1 - product.value.price / product.value.original_price) * 100) : null,
        
      variants: variants.value
    }

    await axios.post('http://localhost:3000/api/admin/products', payload)
    
    alert('🎉 Thêm sản phẩm thành công!')
    router.push('/admin/manage-products')
    
  } catch (error) {
    console.error("Lỗi khi thêm sản phẩm:", error)
    alert('Thêm thất bại. Vui lòng kiểm tra lại Console!')
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
                <input v-model="product.category_id" type="text" placeholder="Nhập mã UUID danh mục..." class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
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
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Tổng Tồn Kho (Phụ)</label>
                <input v-model.number="product.stock_quantity" type="number" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
              </div>
            </div>

            <div class="grid grid-cols-4 gap-5 items-end">
              <div class="col-span-3">
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Link Ảnh Chính</label>
                <input v-model="product.image_url" type="text" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500">
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
            <h3 class="text-lg font-bold text-gray-800">Biến Thể (Tùy chọn)</h3>
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
                <div>
                  <label class="block text-[12px] font-semibold text-gray-600 mb-1">Tồn kho màu</label>
                  <input v-model.number="variant.stock" type="number" class="w-full px-3 py-2 rounded-lg border border-gray-200 text-sm outline-none">
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>
  </div>
</template>