<script setup>
// Import các component sau khi bạn đã tạo chúng
// import HeroBanner from '@/components/home/HeroBanner.vue'
// import FlashSale from '@/components/home/FlashSale.vue'
// import CategoryList from '@/components/home/CategoryList.vue'
// import ProductCard from '@/components/common/ProductCard.vue'
</script>

<template>
  <div class="space-y-12">
    <!-- Banner Section -->
    <section class="bg-black text-white h-80 rounded-xl flex items-center justify-center">
      <h2 class="text-3xl font-bold">Khuyến mãi 10% - Samsung QLED 8K</h2>
      <!-- Component: <HeroBanner /> -->
    </section>

    <!-- Flash Sales Section -->
    <section>
      <div class="flex items-center gap-2 mb-6">
        <div class="w-4 h-8 bg-danger rounded-sm"></div>
        <h2 class="text-xl font-bold text-danger">Flash Sales</h2>
      </div>
      <!-- Grid chứa Product Cards -->
      <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-5 gap-4">
        <!-- Chỗ này sẽ dùng v-for render <ProductCard /> -->
        <div v-for="i in 5" :key="i" class="border p-4 rounded bg-white shadow-sm">
           <div class="h-40 bg-gray-200 mb-4 rounded flex items-center justify-center text-gray-400">Image {{i}}</div>
           <h3 class="text-sm font-medium">Sản phẩm mẫu</h3>
           <p class="text-danger font-bold mt-2">500.000đ</p>
        </div>
      </div>
    </section>

    <!-- Categories Section -->
    <section>
       <h2 class="text-xl font-bold mb-6">Danh Mục</h2>
       <!-- Khung tĩnh -->
       <div class="grid grid-cols-3 md:grid-cols-6 gap-4">
         <div v-for="i in 6" :key="i" class="border py-6 rounded text-center hover:bg-primary hover:text-white transition cursor-pointer">
            <div class="text-2xl mb-2">📱</div>
            <span class="text-sm">Danh mục {{i}}</span>
         </div>
       </div>
    </section>
  </div>
</template>