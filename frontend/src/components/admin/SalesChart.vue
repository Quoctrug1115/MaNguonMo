<script setup>
import { ref } from 'vue'
import { Line } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler // Plugin cực kỳ quan trọng để đổ màu nền dưới đường Line
} from 'chart.js'

// Đăng ký các module cần thiết của Chart.js
ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler 
)

// Dữ liệu mô phỏng giống với bản thiết kế
const chartData = ref({
  labels: ['5k', '10k', '15k', '20k', '25k', '30k', '35k', '40k', '45k', '50k', '55k', '60k'],
  datasets: [
    {
      label: 'Sales Data',
      // Các điểm dữ liệu lên xuống giống hình mẫu
      data: [20, 30, 48, 30, 52, 85, 35, 55, 25, 75, 45, 55], 
      borderColor: '#4379EE', // Màu xanh dương của đường viền
      backgroundColor: 'rgba(67, 121, 238, 0.15)', // Màu nền xanh nhạt đổ bóng phía dưới
      borderWidth: 2,
      pointBackgroundColor: '#4379EE',
      pointBorderColor: '#ffffff',
      pointBorderWidth: 2,
      pointRadius: 4, // Kích thước chấm tròn
      pointHoverRadius: 6,
      fill: true, // KÍCH HOẠT ĐỔ MÀU NỀN
      tension: 0.4 // KÍCH HOẠT ĐƯỜNG CONG MỀM MẠI (0 là thẳng cứng, 0.4 là cong đẹp)
    }
  ]
})

// Cấu hình tuỳ chỉnh giao diện biểu đồ
const chartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false // Ẩn chữ "Sales Data" ở trên cùng (Vì mẫu không có)
    },
    tooltip: {
      backgroundColor: '#4379EE', // Màu nền tooltip khi trỏ chuột vào
      padding: 10,
      cornerRadius: 4,
      displayColors: false,
      callbacks: {
        label: function(context) {
          return context.parsed.y + ' (Sales)';
        }
      }
    }
  },
  scales: {
    y: {
      min: 0,
      max: 100,
      ticks: {
        stepSize: 20,
        color: '#9CA3AF',
        callback: function(value) {
          return value + '%'; // Thêm dấu % vào cột dọc giống mẫu
        }
      },
      grid: {
        color: '#F3F4F6', // Kẻ đường ngang ngang màu xám siêu nhạt
        drawBorder: false
      },
      border: { display: false }
    },
    x: {
      ticks: {
        color: '#9CA3AF'
      },
      grid: {
        display: false, // Ẩn các đường kẻ dọc
        drawBorder: false
      },
      border: { display: false }
    }
  }
})
</script>

<template>
  <div class="w-full h-72">
    <Line :data="chartData" :options="chartOptions" />
  </div>
</template>