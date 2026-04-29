<script setup>
import TheHeader from './components/layout/Header.vue'
import TheFooter from './components/layout/Footer.vue'
</script>

<template>
  <!-- min-h-screen và flex-col đảm bảo Footer luôn bị đẩy xuống đáy trang -->
  <div class="flex flex-col min-h-screen">
    
    <!-- Header dùng chung -->
    <TheHeader />

    <!-- Khu vực hiển thị nội dung các trang -->
    <main class="flex-grow">
      <router-view />
    </main>

    <!-- Footer dùng chung -->
    <TheFooter />
    
  </div>
</template>

<style>
body {
  background-color: #f9fafb; 
}
</style>