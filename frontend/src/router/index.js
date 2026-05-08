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
      path: '/orders',
      name: 'orders',
      component: () => import('../views/OrderHistoryView.vue'),
      meta: { requiresAuth: true }
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
  // 1. Kiểm tra đúng tên chìa khóa là 'token' (giống với lúc Đăng nhập)
  const isAuthenticated = localStorage.getItem('token');

  // 2. Nếu trang đó yêu cầu đăng nhập (requiresAuth) MÀ lại không có token
  if (to.meta.requiresAuth && !isAuthenticated) {
    alert('Bạn cần đăng nhập để truy cập khu vực này!');
    next('/login'); // Đá về trang đăng nhập
  } else {
    next(); // Cho phép đi tiếp
  }
})

export default router