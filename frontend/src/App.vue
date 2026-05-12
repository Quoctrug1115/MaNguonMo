<script setup>
import TheHeader from './components/layout/Header.vue'
import TheFooter from './components/layout/Footer.vue'
// 1. NHỚ IMPORT THÊM ref VÀ onMounted TỪ VUE
import { computed, ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

// 2. KHAI BÁO BIẾN DỮ LIỆU ĐỂ HỨNG ROLE
const userRole = ref('guest')

onMounted(() => {
  userRole.value = localStorage.getItem('role') || 'guest'
})

const isAdminRoute = computed(() => {
  return route.path.startsWith('/admin')
})
</script>

<template>
  <div class="flex flex-col min-h-screen">
    
    <TheHeader v-if="!isAdminRoute"/>

    <div v-if="userRole === 'admin' && !isAdminRoute" class="max-w-7xl mx-auto w-full px-4 mt-4 flex justify-end">
      <router-link
        to="/admin"
        class="bg-blue-600 text-white px-5 py-2.5 rounded-xl font-bold shadow-md hover:bg-blue-700 transition-colors flex items-center gap-2">
        Vào Trang Quản Trị ⚙️
      </router-link>
    </div>

    <main class="flex-grow">
      <router-view />
    </main>

    <TheFooter v-if="!isAdminRoute"/>
    
  </div>
</template>

<style>
body {
  background-color: #f9fafb; 
}
</style>