<script setup>
import ServiceFeatures from '@/components/home/ServiceFeatures.vue'

// Dữ liệu mô phỏng cho khối thống kê
const stats = [
  { id: 1, icon: 'M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6', value: '10.5k', label: 'Sellers active our site', isActive: false },
  { id: 2, icon: 'M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z', value: '33k', label: 'Monthly Product Sale', isActive: true },
  { id: 3, icon: 'M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z', value: '45.5k', label: 'Customer active in our site', isActive: false },
  { id: 4, icon: 'M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z', value: '25k', label: 'Annual gross sale in our site', isActive: false }
]
</script>

<template>
  <div class="container mx-auto px-4 max-w-6xl py-10 mb-10">
    
    <!-- Breadcrumb -->
    <div class="text-sm text-gray-500 mb-12">
      <router-link to="/" class="hover:text-primary transition">Home</router-link> / 
      <span class="text-gray-900">About</span>
    </div>

    <!-- KHU VỰC 1: OUR STORY -->
    <div class="flex flex-col lg:flex-row items-center gap-12 mb-32">
      <!-- Cột Text -->
      <div class="lg:w-1/2 lg:pr-12">
        <h1 class="text-4xl md:text-5xl font-bold mb-8 text-gray-900 tracking-tight">Our Story</h1>
        <div class="text-gray-600 space-y-6 leading-relaxed font-light">
          <p>
            Launched in 2015, Exclusive is South Asia's premier online shopping marketplace with an active presence in Bangladesh. Supported by wide range of tailored marketing, data and service solutions, Exclusive has 10,500 sellers and 300 brands and serves 3 millions customers across the region.
          </p>
          <p>
            Exclusive has more than 1 Million products to offer, growing at a very fast. Exclusive offers a diverse assortment in categories ranging from consumer.
          </p>
        </div>
      </div>
      <!-- Cột Hình ảnh -->
      <div class="lg:w-1/2 flex justify-end">
        <div class="w-full h-[400px] md:h-[500px] bg-pink-100 rounded-lg overflow-hidden flex items-center justify-center border border-gray-100">
          <!-- Thay thế bằng ảnh thật của bạn -->
          <span class="text-pink-300 font-medium">[Hình ảnh 2 cô gái mua sắm]</span>
        </div>
      </div>
    </div>

    <!-- KHU VỰC 2: THỐNG KÊ (STATS) -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 mb-32">
      <div 
        v-for="stat in stats" 
        :key="stat.id"
        :class="[
          'border rounded-lg p-8 flex flex-col items-center justify-center text-center transition-all duration-300 group cursor-pointer shadow-sm hover:shadow-md',
          stat.isActive ? 'bg-primary border-primary text-white' : 'bg-white border-gray-200 text-gray-900 hover:bg-primary hover:border-primary hover:text-white'
        ]"
      >
        <!-- Vòng tròn bọc Icon -->
        <div 
          :class="[
            'w-16 h-16 rounded-full flex items-center justify-center mb-6 border-[8px]',
            stat.isActive ? 'bg-white border-blue-400/30' : 'bg-black border-gray-200 group-hover:bg-white group-hover:border-blue-400/30'
          ]"
        >
          <svg 
            :class="['w-8 h-8', stat.isActive ? 'text-black' : 'text-white group-hover:text-black']" 
            fill="none" stroke="currentColor" viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" :d="stat.icon" />
          </svg>
        </div>
        
        <!-- Giá trị & Nhãn -->
        <h3 class="text-3xl font-bold mb-2">{{ stat.value }}</h3>
        <p :class="['text-sm', stat.isActive ? 'text-blue-100' : 'text-gray-500 group-hover:text-blue-100']">
          {{ stat.label }}
        </p>
      </div>
    </div>

    <!-- KHU VỰC 3: DỊCH VỤ KHÁCH HÀNG (Tái sử dụng Component) -->
    <ServiceFeatures />

  </div>
</template>