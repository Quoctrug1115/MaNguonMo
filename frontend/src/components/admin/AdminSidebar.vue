<script setup>

</script>

<template>
  <aside class="w-64 bg-white border-r border-gray-100 flex flex-col flex-shrink-0">
    <div class="h-20 flex items-center justify-center border-b border-gray-50 mb-4">
      <h1 class="text-xl font-bold text-blue-600 tracking-wide text-center leading-tight">
        Cửa Hàng<br><span class="text-gray-900">Điện Máy</span>
      </h1>
    </div>

    <nav class="flex-1 overflow-y-auto py-4">
      <ul class="space-y-2 px-4">
        
        <li>
          <router-link 
            to="/admin" 
            exact-active-class="bg-blue-500 text-white shadow-sm shadow-blue-200"
            class="flex items-center gap-3 px-4 py-3 rounded-lg font-medium transition-colors text-gray-600 hover:text-blue-600 hover:bg-blue-50">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/></svg>
            Dashboard
          </router-link>
        </li>

        <li>
          <router-link 
            to="/admin/products" 
            active-class="bg-blue-500 text-white shadow-sm shadow-blue-200"
            class="flex items-center gap-3 px-4 py-3 rounded-lg font-medium transition-colors text-gray-600 hover:text-blue-600 hover:bg-blue-50"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>
            Sản Phẩm
          </router-link>
        </li>

        <li>
          <a href="#" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>
            Sản Phẩm Bán Chạy
          </a>
        </li>
        <li>
          <a href="#" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"/></svg>
            Tin Nhắn
          </a>
        </li>
        <li>
          <a href="#" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"/></svg>
            Đơn Hàng
          </a>
        </li>
        <li>
          <router-link 
            to="/admin/manage-products" 
            active-class="bg-blue-500 text-white shadow-sm shadow-blue-200"
            class="flex items-center gap-3 px-4 py-3 rounded-lg font-medium transition-colors text-gray-600 hover:text-blue-600 hover:bg-blue-50">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"/></svg>
            Quản Lý Sản Phẩm
          </router-link>
        </li>
      </ul>

      <div class="mt-8 px-4">
        <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-4 px-4">Pages</p>
        <ul class="space-y-2">
          <li>
            <a href="#" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
              Setting
            </a>
          </li>
          <li>
            <a href="#" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg>
              Tài Khoản
            </a>
          </li>
          <li>
            <a href="#" class="flex items-center gap-3 text-gray-600 hover:text-red-600 hover:bg-red-50 px-4 py-3 rounded-lg font-medium transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/></svg>
              Logout
            </a>
          </li>
        </ul>
      </div>
    </nav>
  </aside>
</template>