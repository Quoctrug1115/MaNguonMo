<script setup>
import { ref } from 'vue'

const isProfileMenuOpen = ref(false)
const isNotificationOpen = ref(false)
</script>

<template>
  <div class="flex h-screen bg-[#F5F6FA] font-sans text-gray-800 overflow-hidden">
    
    <aside class="w-64 bg-white border-r border-gray-100 flex flex-col flex-shrink-0">
      <div class="h-20 flex items-center justify-center border-b border-gray-50 mb-4">
        <h1 class="text-xl font-bold text-blue-600 tracking-wide text-center leading-tight">
          Cửa Hàng<br><span class="text-gray-900">Điện Máy</span>
        </h1>
      </div>

      <nav class="flex-1 overflow-y-auto py-4">
        <ul class="space-y-2 px-4">
          <li>
            <router-link to="/admin" class="flex items-center gap-3 bg-blue-500 text-white px-4 py-3 rounded-lg font-medium shadow-sm shadow-blue-200">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/></svg>
              Dashboard
            </router-link>
          </li>
          <li>
            <router-link to="/admin/products" exact-active-class="bg-blue-500 text-white shadow-sm shadow-blue-200" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>
              Sản Phẩm
            </router-link>
          </li>
          <li>
            <router-link to="/admin/popular-products" exact-active-class="bg-blue-500 text-white shadow-sm shadow-blue-200" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>
              Sản Phẩm Bán Chạy
            </router-link>
          </li>
          <li>
            <router-link to="/admin/messages" exact-active-class="bg-blue-500 text-white shadow-sm shadow-blue-200" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z"/></svg>
              Tin Nhắn
            </router-link>
          </li>
          <li>
            <a href="#" class="flex items-center gap-3 text-gray-600 hover:text-blue-600 hover:bg-blue-50 px-4 py-3 rounded-lg font-medium transition-colors">
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01"/></svg>
              Đơn Hàng
            </a>
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

    <div class="flex-1 flex flex-col overflow-hidden">
      
      <header class="h-20 bg-[#F5F6FA] flex items-center justify-between px-8">
        <div class="relative w-96">
          <span class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
            <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>
          </span>
          <input type="text" placeholder="Search" class="w-full bg-white border-none rounded-full pl-10 pr-4 py-2.5 text-sm outline-none focus:ring-2 focus:ring-blue-100 shadow-sm" />
        </div>

        <div class="flex items-center gap-6">
          
          <div class="relative">
            <button @click="isNotificationOpen = !isNotificationOpen" class="relative p-2 text-gray-400 hover:text-gray-600 transition-colors focus:outline-none">
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/></svg>
              <span class="absolute top-1.5 right-2 w-2 h-2 bg-red-500 rounded-full border border-white"></span>
            </button>

            <transition 
              enter-active-class="transition duration-200 ease-out" 
              enter-from-class="transform scale-95 opacity-0 translate-y-2" 
              enter-to-class="transform scale-100 opacity-100 translate-y-0" 
              leave-active-class="transition duration-150 ease-in" 
              leave-from-class="transform scale-100 opacity-100 translate-y-0" 
              leave-to-class="transform scale-95 opacity-0 translate-y-2"
            >
              <div v-show="isNotificationOpen" class="absolute right-0 mt-3 w-80 bg-white rounded-xl shadow-[0_10px_40px_rgba(0,0,0,0.08)] py-2 border border-gray-100 z-50">
                
                <div class="px-5 py-3 border-b border-gray-50 mb-1">
                  <h3 class="text-sm font-bold text-gray-800">Notification</h3>
                </div>

                <div class="max-h-[320px] overflow-y-auto">
                  
                  <a href="#" class="flex items-start gap-4 px-5 py-3 hover:bg-gray-50 transition-colors">
                    <div class="w-10 h-10 rounded-full bg-blue-50 flex items-center justify-center flex-shrink-0 text-blue-500 mt-0.5">
                      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/></svg>
                    </div>
                    <div>
                      <p class="text-sm font-bold text-gray-800">Settings</p>
                      <p class="text-[13px] text-gray-500 mt-0.5">Update Dashboard</p>
                    </div>
                  </a>

                  <a href="#" class="flex items-start gap-4 px-5 py-3 hover:bg-gray-50 transition-colors">
                    <div class="w-10 h-10 rounded-full bg-pink-50 flex items-center justify-center flex-shrink-0 text-pink-500 mt-0.5">
                      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/></svg>
                    </div>
                    <div>
                      <p class="text-sm font-bold text-gray-800">Event Update</p>
                      <p class="text-[13px] text-gray-500 mt-0.5">An event date update again</p>
                    </div>
                  </a>

                  <a href="#" class="flex items-start gap-4 px-5 py-3 hover:bg-gray-50 transition-colors">
                    <div class="w-10 h-10 rounded-full bg-purple-50 flex items-center justify-center flex-shrink-0 text-purple-500 mt-0.5">
                      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg>
                    </div>
                    <div>
                      <p class="text-sm font-bold text-gray-800">Profile</p>
                      <p class="text-[13px] text-gray-500 mt-0.5">Update your profile</p>
                    </div>
                  </a>

                  <a href="#" class="flex items-start gap-4 px-5 py-3 hover:bg-gray-50 transition-colors">
                    <div class="w-10 h-10 rounded-full bg-red-50 flex items-center justify-center flex-shrink-0 text-red-500 mt-0.5">
                      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/></svg>
                    </div>
                    <div>
                      <p class="text-sm font-bold text-gray-800">Application Error</p>
                      <p class="text-[13px] text-gray-500 mt-0.5">Check Your running application</p>
                    </div>
                  </a>

                </div>

                <div class="border-t border-gray-50 mt-1">
                  <a href="#" class="block text-center px-4 py-3.5 text-xs font-bold text-gray-400 hover:text-blue-600 transition-colors">
                    See all notification
                  </a>
                </div>

              </div>
            </transition>
          </div>

          <div class="flex items-center gap-2 cursor-pointer">
            <img src="https://flagcdn.com/w20/gb.png" alt="English" class="w-5 h-auto rounded-sm" />
            <span class="text-sm font-medium text-gray-600">English</span>
            <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
          </div>

          <div class="relative">
            <div @click="isProfileMenuOpen = !isProfileMenuOpen" class="flex items-center gap-3 cursor-pointer">
              <img src="https://i.pravatar.cc/150?img=47" alt="Admin" class="w-10 h-10 rounded-full object-cover shadow-sm border-2 border-white" />
              <div class="hidden md:block text-left">
                <p class="text-sm font-bold text-gray-800 leading-none">Moni Roy</p>
                <p class="text-xs text-gray-500 mt-1">Admin</p>
              </div>
              <svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/></svg>
            </div>

            <div v-show="isProfileMenuOpen" class="absolute right-0 mt-3 w-48 bg-white rounded-xl shadow-lg py-2 border border-gray-100 z-50">
              <a href="#" class="flex items-center gap-3 px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 hover:text-blue-600"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg> Manage Account</a>
              <a href="#" class="flex items-center gap-3 px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 hover:text-blue-600"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/></svg> Change Password</a>
              <a href="#" class="flex items-center gap-3 px-4 py-2 text-sm text-gray-700 hover:bg-gray-50 hover:text-blue-600"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/></svg> Activity Log</a>
              <div class="border-t border-gray-100 my-1"></div>
              <a href="#" class="flex items-center gap-3 px-4 py-2 text-sm text-red-600 hover:bg-red-50"><svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"/></svg> Log out</a>
            </div>
          </div>
        </div>
      </header>

      <main class="flex-1 overflow-x-hidden overflow-y-auto bg-[#F5F6FA] px-8 pb-8">
        <router-view />
      </main>

    </div>
  </div>
</template>