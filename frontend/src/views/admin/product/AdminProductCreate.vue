<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

const router = useRouter()
const isLoading = ref(false)
const isUploading = ref(false)
const uploadedGallery = ref([])
const categoriesList = ref([])

const fetchCategories = async () => {
  try {
    const res = await axios.get('http://localhost:3000/api/categories')
    categoriesList.value = res.data // Gán dữ liệu trả về vào mảng
  } catch (error) {
    console.error("Lỗi khi tải danh mục:", error)
  }
}

onMounted(() => {
  fetchCategories()
})

// 1. Thông tin cơ bản
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

// 2. Dữ liệu Biến thể màu
const variants = ref([
  { color_name: '', color_hex: '#000000', stock: 0, image_url: '' }
])
const addVariant = () => { variants.value.push({ color_name: '', color_hex: '#ffffff', stock: 0, image_url: '' }) }
const removeVariant = (index) => { if (variants.value.length > 1) variants.value.splice(index, 1); else alert('Phải có ít nhất 1 màu!') }

// 3. DỮ LIỆU THÔNG SỐ KỸ THUẬT (MỚI)
const specifications = ref([
  { spec_name: '', spec_value: '' }
])
const addSpecification = () => { specifications.value.push({ spec_name: '', spec_value: '' }) }
const removeSpecification = (index) => { specifications.value.splice(index, 1) }


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

// Hàm Submit Sản Phẩm
const submitProduct = async () => {
  try {
    isLoading.value = true
    
    // 1. LẤY TOKEN
    const token = localStorage.getItem('token')
    if (!token) {
      alert('Bạn chưa đăng nhập hoặc phiên đã hết hạn. Vui lòng đăng nhập lại!')
      return
    }

    // 2. Lọc thông số rỗng
    const validSpecs = specifications.value.filter(s => s.spec_name.trim() !== '' && s.spec_value.trim() !== '')
    // 3. Đóng gói dữ liệu an toàn
const payload = {
      name: product.value.name,
      description: product.value.description || null,
      
      // Ép kiểu chắc chắn là số
      price: Number(product.value.price) || 0,
      original_price: product.value.original_price ? Number(product.value.original_price) : null,
      stock_quantity: Number(product.value.stock_quantity) || 0,
      
      image_url: product.value.image_url || null,
      is_new: Boolean(product.value.is_new),
      rating: null,
      reviews_count: null,
      
      // MẸO: Kiểm tra sơ bộ xem có đúng định dạng độ dài của UUID không (36 ký tự)
      // Nếu nhập linh tinh, hệ thống sẽ tự động chuyển thành null để không bị lỗi 422
      category_id: (product.value.category_id && product.value.category_id.length === 36) 
                   ? product.value.category_id 
                   : null,
                   
      discount_percent: product.value.original_price 
                        ? Math.round((1 - Number(product.value.price) / Number(product.value.original_price)) * 100) 
                        : null,
                        
      // Ép kiểu các biến thể màu sắc
      variants: variants.value.map(v => ({
          color_name: v.color_name,
          color_hex: v.color_hex,
          stock: Number(v.stock) || 0,
          image_url: v.image_url || null
      })),
      
      specifications: validSpecs.length > 0 ? validSpecs : null
    }

    // Console log ra để bạn tự kiểm tra trước khi gửi
    console.log("Dữ liệu chuẩn bị gửi đi:", payload);

    // GỌI API
    await axios.post('http://localhost:3000/api/admin/products', payload, {
      headers: { 'Authorization': `Bearer ${token}` }
    })

    alert('🎉 Thêm sản phẩm thành công!')
    router.push('/admin/manage-products')
    
  } catch (error) {
    // 5. BẮT LỖI THÔNG MINH ĐỂ DỄ DÀNG SỬA CHỮA
    if (error.response) {
      const status = error.response.status;
      if (status === 401) {
         alert('Token không hợp lệ hoặc đã cũ. Vui lòng ĐĂNG XUẤT và ĐĂNG NHẬP LẠI!')
      } else if (status === 403) {
         alert('Truy cập bị từ chối! Tài khoản này không có quyền Admin.')
      } else {
         // Hiển thị trực tiếp lỗi từ Backend Rust trả về (nếu có)
         alert(`Lỗi hệ thống: ${error.response.data?.error || 'Vui lòng kiểm tra lại dữ liệu nhập'}`)
      }
    } else {
      alert('Không thể kết nối đến máy chủ!')
    }
    console.error(error)
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
          <button @click="copyToClipboard(url)" class="absolute inset-0 bg-black/50 text-white text-xs font-bold opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity cursor-pointer">
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
                <label class="block text-[13px] font-semibold text-gray-700 mb-1">Danh mục sản phẩm</label>
                  <select v-model="product.category_id" class="w-full px-4 py-2 rounded-xl border border-gray-200 bg-gray-50 outline-none focus:border-blue-500 cursor-pointer">
                    <option :value="null">Vui lòng chọn danh mục</option>
                    <option v-for="cat in categoriesList" :key="cat.id" :value="cat.id">{{ cat.name }}</option>
                </select>
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

            <div class="mt-8 pt-6 border-t border-gray-100">
              <div class="flex justify-between items-center mb-4">
                <div>
                  <h4 class="text-[15px] font-bold text-gray-800">Thông Số Kỹ Thuật</h4>
                  <p class="text-xs text-gray-500 mt-1">Các thông tin cấu hình chi tiết (Ví dụ: RAM, Chip, Màn hình...)</p>
                </div>
                <button @click="addSpecification" class="text-[12px] font-bold text-blue-600 hover:bg-blue-50 px-3 py-1.5 rounded-lg border border-blue-100 transition-colors">
                  + Thêm thông số
                </button>
              </div>

              <div class="space-y-3">
                <div v-for="(spec, index) in specifications" :key="index" class="flex gap-3 items-center bg-gray-50 p-2 rounded-xl border border-gray-100">
                  <input v-model="spec.spec_name" type="text" placeholder="Tên thông số (VD: RAM)" class="w-1/3 px-3 py-2 rounded-lg border border-gray-200 bg-white text-sm outline-none focus:border-blue-500">
                  <input v-model="spec.spec_value" type="text" placeholder="Giá trị (VD: 16GB)" class="flex-grow px-3 py-2 rounded-lg border border-gray-200 bg-white text-sm outline-none focus:border-blue-500">
                  <button @click="removeSpecification(index)" title="Xóa" class="text-gray-400 hover:text-red-500 p-2 bg-white rounded-lg border border-gray-200 shadow-sm">
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
                  </button>
                </div>
              </div>
            </div>
            </div>
        </div>
      </div>

      <div class="col-span-1">
        <div class="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <div class="flex justify-between items-center mb-5">
            <h3 class="text-lg font-bold text-gray-800">Biến Thể Màu Sắc</h3>
            <button @click="addVariant" class="text-[13px] font-bold text-blue-600 hover:bg-blue-50 border border-blue-100 px-3 py-1.5 rounded-lg">+ Thêm Màu</button>
          </div>

          <div class="space-y-4 max-h-[800px] overflow-y-auto pr-2">
            <div v-for="(variant, index) in variants" :key="index" class="p-4 rounded-xl border border-gray-100 bg-gray-50 relative">
              <button @click="removeVariant(index)" class="absolute top-2 right-2 text-gray-400 hover:text-red-500 p-1">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/></svg>
              </button>
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