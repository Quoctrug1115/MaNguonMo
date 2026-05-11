import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/client/homepage/HomeView.vue'

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
      component: () => import('../views/auth/LoginView.vue')
    },
    {
      path: '/profile',
      name: 'profile',
      component: () => import('../views/client/user/ProfileView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/register',
      name: 'register',
      component: () => import('../views/auth/RegisterView.vue')
    },
    {
      path: '/forgot-password',
      name: 'forgot-password',
      component: () => import('../views/auth/ForgotPasswordView.vue')
    },
    {
      path: '/wishlist',
      name: 'wishlist',
      component: () => import('../views/client/user/WishlistView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/cart',
      name: 'cart',
      component: () => import('../views/client/orderandcart/CartView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/checkout',
      name: 'checkout',
      component: () => import('../views/client/orderandcart/CheckoutView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/client/homepage/AboutView.vue')
    },
    {
      path: '/contact',
      name: 'contact',
      component: () => import('../views/client/homepage/ContactView.vue')
    },
    {
      path: '/product/:id',
      name: 'product-detail',
      component: () => import('../views/client/product/ProductDetailView.vue'),
      props: true // Cho phép truyền ID như một prop vào component
    },
    {
      path: '/products',
      name: 'products',
      component: () => import('../views/client/product/ProductsView.vue')
    },
    {
      path: '/orders',
      name: 'orders',
      component: () => import('../views/client/user/OrderHistoryView.vue'),
      meta: { requiresAuth: true }
    },
    {
      path: '/admin',
      component: () => import('../layouts/AdminLayout.vue'), // Trỏ vào Layout Admin
      children: [
        {
          path: '', // Khi đường dẫn là /admin, tự động render Dashboard
          name: 'admin-dashboard',
          component: () => import('../views/admin/AdminDashboard.vue')
        },
        {
          path: 'products',
          name: 'admin-products',
          component: () => import('../views/admin/AdminProductsView.vue')
        },
        {
          path: 'manage-products',
          name: 'admin-product-list',
          component: () => import('../views/admin/AdminProductList.vue')
        }
      ]
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