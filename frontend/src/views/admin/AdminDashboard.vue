<script setup>
import SalesChart from '@/components/admin/SalesChart.vue'
</script>

<template>
  <div>
    <h2 class="text-2xl font-bold text-gray-800 mb-6">Dashboard</h2>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      
      <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-50 flex flex-col justify-between">
        <div class="flex justify-between items-start mb-4">
          <div>
            <p class="text-sm font-medium text-gray-500 mb-1">Total User</p>
            <h3 class="text-2xl font-bold text-gray-800">40,689</h3>
          </div>
          <div class="w-12 h-12 rounded-2xl bg-indigo-100 flex items-center justify-center text-indigo-500">
            <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20"><path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z"/></svg>
          </div>
        </div>
        <p class="text-sm font-medium text-green-500 flex items-center gap-1">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>
          8.5% <span class="text-gray-400 font-normal ml-1">Up from yesterday</span>
        </p>
      </div>

      <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-50 flex flex-col justify-between">
        <div class="flex justify-between items-start mb-4">
          <div>
            <p class="text-sm font-medium text-gray-500 mb-1">Total Order</p>
            <h3 class="text-2xl font-bold text-gray-800">10293</h3>
          </div>
          <div class="w-12 h-12 rounded-2xl bg-yellow-100 flex items-center justify-center text-yellow-600">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"/></svg>
          </div>
        </div>
        <p class="text-sm font-medium text-green-500 flex items-center gap-1">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>
          1.3% <span class="text-gray-400 font-normal ml-1">Up from past week</span>
        </p>
      </div>

      <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-50 flex flex-col justify-between">
        <div class="flex justify-between items-start mb-4">
          <div>
            <p class="text-sm font-medium text-gray-500 mb-1">Total Sales</p>
            <h3 class="text-2xl font-bold text-gray-800">$89,000</h3>
          </div>
          <div class="w-12 h-12 rounded-2xl bg-green-100 flex items-center justify-center text-green-600">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"/></svg>
          </div>
        </div>
        <p class="text-sm font-medium text-red-500 flex items-center gap-1">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 17h8m0 0v-8m0 8l-8-8-4 4-6-6"/></svg>
          4.3% <span class="text-gray-400 font-normal ml-1">Down from yesterday</span>
        </p>
      </div>

      <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-50 flex flex-col justify-between">
        <div class="flex justify-between items-start mb-4">
          <div>
            <p class="text-sm font-medium text-gray-500 mb-1">Total Pending</p>
            <h3 class="text-2xl font-bold text-gray-800">2040</h3>
          </div>
          <div class="w-12 h-12 rounded-2xl bg-orange-100 flex items-center justify-center text-orange-500">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
          </div>
        </div>
        <p class="text-sm font-medium text-green-500 flex items-center gap-1">
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/></svg>
          1.8% <span class="text-gray-400 font-normal ml-1">Up from yesterday</span>
        </p>
      </div>
    </div>

    <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-50 mb-8">
      <div class="flex justify-between items-center mb-6">
        <h3 class="text-lg font-bold text-gray-800">Sales Details</h3>
        <select class="border border-gray-200 text-gray-600 rounded-lg px-3 py-1.5 text-sm outline-none focus:border-blue-500">
          <option>October</option>
          <option>November</option>
        </select>
      </div>
    
      <SalesChart />

      <div class="h-72 w-full bg-gray-50 rounded-lg border border-dashed border-gray-200 flex flex-col items-center justify-center text-gray-400">
        <svg class="w-10 h-10 mb-2 text-blue-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"/></svg>
        <p>Khu vực tích hợp Biểu đồ Line Chart (Sales Data)</p>
      </div>
    </div>

    <div class="bg-white rounded-2xl p-6 shadow-sm border border-gray-50">
      <div class="flex justify-between items-center mb-6">
        <h3 class="text-lg font-bold text-gray-800">Deals Details</h3>
        <select class="border border-gray-200 text-gray-600 rounded-lg px-3 py-1.5 text-sm outline-none focus:border-blue-500">
          <option>October</option>
        </select>
      </div>
      
      <div class="overflow-x-auto">
        <table class="w-full text-left border-collapse">
          <thead>
            <tr class="bg-gray-50 text-gray-500 text-xs uppercase tracking-wider">
              <th class="px-4 py-3 rounded-l-lg font-medium">Product Name</th>
              <th class="px-4 py-3 font-medium">Location</th>
              <th class="px-4 py-3 font-medium">Date - Time</th>
              <th class="px-4 py-3 font-medium">Piece</th>
              <th class="px-4 py-3 font-medium">Amount</th>
              <th class="px-4 py-3 rounded-r-lg font-medium">Status</th>
            </tr>
          </thead>
          <tbody class="text-sm">
            <tr class="border-b border-gray-50 hover:bg-gray-50 transition-colors">
            <td class="px-4 py-4 flex items-center gap-3">
                <img src="https://via.placeholder.com/40" class="w-8 h-8 rounded-md bg-gray-100 object-cover" alt="Apple Watch" />
                <span class="font-medium text-gray-800">Apple Watch</span>
              </td>
              <td class="px-4 py-4 text-gray-600">6096 Marjolaine Landing</td>
              <td class="px-4 py-4 text-gray-600">12.09.2026 - 12:53 PM</td>
              <td class="px-4 py-4 text-gray-600">423</td>
              <td class="px-4 py-4 font-medium text-gray-800">$34,295</td>
              <td class="px-4 py-4">
                <span class="bg-teal-100 text-teal-600 px-3 py-1 rounded-full text-xs font-bold tracking-wide">Delivered</span>
              </td>
            </tr>
            </tbody>
        </table>
      </div>
    </div>

  </div>
</template>