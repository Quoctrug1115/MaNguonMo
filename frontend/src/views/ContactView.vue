<script setup>
import { reactive } from 'vue'

const contactForm = reactive({
  name: '',
  email: '',
  phone: '',
  message: ''
})

const handleSendMessage = () => {
  console.log('Dữ liệu form liên hệ:', contactForm)
  // TODO: Gọi API backend Rust Axum để gửi email/lưu tin nhắn hỗ trợ
  alert('Tin nhắn của bạn đã được gửi. Chúng tôi sẽ phản hồi sớm nhất!')
  
  // Reset form sau khi gửi
  contactForm.name = ''
  contactForm.email = ''
  contactForm.phone = ''
  contactForm.message = ''
}
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-20">
    
    <!-- Breadcrumb -->
    <div class="text-sm text-gray-500 mb-12">
      <router-link to="/" class="hover:text-primary transition">Home</router-link> / 
      <span class="text-gray-900 font-medium">Contact</span>
    </div>

    <!-- Main Content: Chia 2 cột -->
    <div class="flex flex-col lg:flex-row gap-8">
      
      <!-- CỘT TRÁI: THÔNG TIN LIÊN HỆ -->
      <div class="lg:w-1/3 bg-white shadow-sm border border-gray-100 rounded-lg p-8 md:p-10">
        
        <!-- Call To Us -->
        <div class="mb-8">
          <div class="flex items-center gap-4 mb-6">
            <div class="w-10 h-10 rounded-full bg-indigo-600 text-white flex items-center justify-center">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
              </svg>
            </div>
            <h3 class="font-bold text-gray-900 text-lg">Số điện thoại cửa hàng</h3>
          </div>
          <div class="space-y-4 text-sm text-gray-700">
            <p>Hoạt động 24/7, 7 ngày trong tuần.</p>
            <p>Phone: +8801611112222</p>
          </div>
        </div>

        <hr class="border-gray-200 mb-8" />

        <!-- Write To Us -->
        <div>
          <div class="flex items-center gap-4 mb-6">
            <div class="w-10 h-10 rounded-full bg-indigo-600 text-white flex items-center justify-center">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
              </svg>
            </div>
            <h3 class="font-bold text-gray-900 text-lg">Gửi tin nhắn cho chúng tôi</h3>
          </div>
          <div class="space-y-4 text-sm text-gray-700">
            <p>Điền vào biểu mẫu của chúng tôi và chúng tôi sẽ liên hệ với bạn trong vòng 24 giờ.</p>
            <p>Emails: customer@exclusive.com</p>
            <p>Emails: support@exclusive.com</p>
          </div>
        </div>

      </div>

      <!-- CỘT PHẢI: FORM GỬI TIN NHẮN -->
      <div class="lg:w-2/3 bg-white shadow-sm border border-gray-100 rounded-lg p-8 md:p-10">
        <form @submit.prevent="handleSendMessage" class="flex flex-col h-full">
          
          <!-- Hàng 3 Input -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
            <div>
              <input 
                type="text" 
                v-model="contactForm.name"
                placeholder="Tên" 
                class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm text-gray-700" 
              />
            </div>
            <div>
              <input 
                type="email" 
                v-model="contactForm.email"
                placeholder="Email *" 
                required
                class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm text-gray-700" 
              />
            </div>
            <div>
              <input 
                type="tel" 
                v-model="contactForm.phone"
                placeholder="SĐT *" 
                required
                class="w-full bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm text-gray-700" 
              />
            </div>
          </div>

          <!-- Textarea -->
          <div class="flex-grow mb-6">
            <textarea 
              v-model="contactForm.message"
              placeholder="bạn cần hỗ trợ gì ?" 
              rows="8"
              class="w-full h-full min-h-[200px] bg-[#f5f5f5] rounded px-4 py-3 focus:outline-none focus:ring-1 focus:ring-primary text-sm text-gray-700 resize-none"
            ></textarea>
          </div>

          <!-- Nút Submit -->
          <div class="flex justify-end">
            <button 
              type="submit" 
              class="bg-primary text-white px-10 py-3 rounded text-sm font-medium hover:bg-blue-600 transition"
            >
              Gửi tin nhắn
            </button>
          </div>

        </form>
      </div>

    </div>
  </div>
</template>