import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  // Sử dụng Web History API để URL trông sạch đẹp (không có dấu #)
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/login',
      name: 'login',
      // Lazy-loading: Component chỉ được tải khi người dùng vào trang này (tối ưu hiệu năng)
      component: () => import('../views/LoginView.vue')
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('../views/RegisterView.vue')
    },
    {
      path: '/forgot-password',
      name: 'forgot-password',
      component: () => import('../views/ForgotPasswordView.vue')
    },
    {
      path: '/wishlist',
      name: 'wishlist',
      component: () => import('../views/WishlistView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/cart',
      name: 'cart',
      component: () => import('../views/CartView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/checkout',
      name: 'checkout',
      component: () => import('../views/CheckoutView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/account',
      name: 'account',
      component: () => import('../views/AccountView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue')
    },
    {
      path: '/contact',
      name: 'contact',
      component: () => import('../views/ContactView.vue')
    },
    {
      path: '/product/:id',
      name: 'product-detail',
      component: () => import('../views/ProductDetailView.vue'),
      props: true // Cho phép truyền ID như một prop vào component
    },
    {
      path: '/products',
      name: 'products',
      component: () => import('../views/ProductsView.vue')
    },
    {
      // Cú pháp catch-all của Vue Router 4
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('../views/NotFoundView.vue')
    }
  ]
})


// TRẠM KIỂM SOÁT HÀNG RÀO FRONTEND
router.beforeEach((to, from, next) => {
  // 1. Kiểm tra xem người dùng đang có thẻ thông hành (token) không
  const isAuthenticated = localStorage.getItem('user_token') !== null

  // 2. Kiểm tra xem trang họ muốn vào (to) có yêu cầu thẻ không
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth)

  if (requiresAuth && !isAuthenticated) {
    // Nếu trang bắt buộc đăng nhập MÀ lại chưa đăng nhập -> Đuổi về trang Login
    alert('Bạn cần đăng nhập để truy cập khu vực này!')
    next('/login')
  }
  else if ((to.path === '/login' || to.path === '/register') && isAuthenticated) {
    // (Tùy chọn) Nếu ĐÃ đăng nhập rồi mà còn cố tình vào trang Login/Register -> Đẩy về Trang chủ
    next('/')
  }
  else {
    // Nếu hợp lệ -> Mở cổng cho đi tiếp
    next()
  }
})

export default router