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
        },
        { 
          path: 'create-product',
          name: 'admin-product-create',
          component: () => import('../views/admin/AdminProductCreate.vue')
        },
        {
          path: 'edit-product/:id', // Có tham số :id
          name: 'admin-product-edit',
          component: () => import('../views/admin/AdminProductEdit.vue'),
          meta: { requiresAdmin: true }
        },
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
  const token = localStorage.getItem('token')
  const role = localStorage.getItem('role')

  // Kiểm tra xem trang người dùng muốn vào (to) có cần quyền Admin không?
  if (to.matched.some(record => record.meta.requiresAdmin)) {
    
    // 1. Nếu chưa đăng nhập -> Đá về trang đăng nhập
    if (!token) {
      alert('Vui lòng đăng nhập để tiếp tục!')
      next({ path: '/login' })
    } 
    // 2. Nếu đã đăng nhập nhưng KHÔNG PHẢI Admin -> Đá về trang chủ
    else if (role !== 'admin') {
      alert('Truy cập bị từ chối! Bạn không có quyền quản trị viên.')
      next({ path: '/' }) // Chuyển về trang chủ của E-commerce
    } 
    // 3. Nếu là Admin -> Mời vào
    else {
      next() 
    }
  } else {
    // Nếu trang không yêu cầu quyền (như trang chủ, đăng nhập) -> Cứ cho vào bình thường
    next()
  }
})

export default router