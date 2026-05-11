<script setup>
import TheHeader from './components/layout/Header.vue'
import TheFooter from './components/layout/Footer.vue'
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const isAdminRoute = computed(() => {
  return route.path.startsWith('/admin')
})
</script>

<template>
  <!-- min-h-screen và flex-col đảm bảo Footer luôn bị đẩy xuống đáy trang -->
  <div class="flex flex-col min-h-screen">
    
    <!-- Header dùng chung -->
    <TheHeader v-if="!isAdminRoute"/>

    <!-- Khu vực hiển thị nội dung các trang -->
    <main class="flex-grow">
      <router-view />
    </main>

    <!-- Footer dùng chung -->
    <TheFooter v-if="!isAdminRoute"/>
    
  </div>
</template>

<style>
body {
  background-color: #f9fafb; 
}
</style>