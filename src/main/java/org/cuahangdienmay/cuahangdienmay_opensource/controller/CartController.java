package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import jakarta.servlet.http.HttpSession;
import org.cuahangdienmay.cuahangdienmay_opensource.model.CartItem;
import org.cuahangdienmay.cuahangdienmay_opensource.model.Product;
import org.cuahangdienmay.cuahangdienmay_opensource.repository.ProductRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam; // ĐÂY LÀ DÒNG BẠN BỊ THIẾU NÀY!

import java.util.ArrayList;
import java.util.List;

@Controller
public class CartController {

    @Autowired
    private ProductRepository productRepository;

    // 1. XEM GIỎ HÀNG
    @GetMapping("/cart")
    public String viewCart(HttpSession session, Model model) {
        model.addAttribute("title", "Giỏ hàng của bạn - Cửa Hàng Điện Máy");

        @SuppressWarnings("unchecked")
        List<CartItem> cart = (List<CartItem>) session.getAttribute("cart");
        if (cart == null) {
            cart = new ArrayList<>(); 
        }

        model.addAttribute("cartItems", cart);

        // Tính tổng tiền an toàn
        double subtotal = 0;
        for (CartItem item : cart) {
            if (item.getPrice() != null) {
                subtotal += item.getPrice() * item.getQuantity();
            }
        }
        model.addAttribute("subtotal", subtotal);

        return "cart";
    }

    // 2. THÊM VÀO GIỎ HÀNG
    @PostMapping("/cart/add/{id}")
    public String addToCart(@PathVariable("id") Long id, HttpSession session) {
        Product product = productRepository.findById(id).orElse(null);
        if (product == null) return "redirect:/";

        @SuppressWarnings("unchecked")
        List<CartItem> cart = (List<CartItem>) session.getAttribute("cart");
        if (cart == null) {
            cart = new ArrayList<>();
        }

        boolean exists = false;
        for (CartItem item : cart) {
            if (item.getProductId().equals(id)) {
                item.setQuantity(item.getQuantity() + 1);
                exists = true;
                break;
            }
        }

        if (!exists) {
            cart.add(new CartItem(product.getId(), product.getName(), product.getPrice(), 1, product.getImg()));
        }

        session.setAttribute("cart", cart);
        return "redirect:/cart"; 
    }

    // 3. XÓA KHỎI GIỎ HÀNG
    @GetMapping("/cart/remove/{id}")
    public String removeFromCart(@PathVariable("id") Long id, HttpSession session) {
        @SuppressWarnings("unchecked")
        List<CartItem> cart = (List<CartItem>) session.getAttribute("cart");
        if (cart != null) {
            cart.removeIf(item -> item.getProductId().equals(id));
            session.setAttribute("cart", cart);
        }
        return "redirect:/cart";
    }

    // 4. CẬP NHẬT SỐ LƯỢNG (Tự động lưu khi thay đổi)
    @PostMapping("/cart/update/{id}")
    public String updateQuantity(@PathVariable("id") Long id, @RequestParam("quantity") int quantity, HttpSession session) {
        @SuppressWarnings("unchecked")
        List<CartItem> cart = (List<CartItem>) session.getAttribute("cart");
        
        if (cart != null) {
            for (CartItem item : cart) {
                if (item.getProductId().equals(id)) {
                    if (quantity <= 0) {
                        cart.remove(item);
                    } else {
                        item.setQuantity(quantity);
                    }
                    break;
                }
            }
            session.setAttribute("cart", cart);
        }
        return "redirect:/cart";
    }

    // 5. XÓA TOÀN BỘ GIỎ HÀNG
    @GetMapping("/cart/clear")
    public String clearCart(HttpSession session) {
        session.removeAttribute("cart");
        return "redirect:/cart";
    }

    // 6. HIỂN THỊ TRANG THANH TOÁN (CHECKOUT)
    @GetMapping("/checkout")
    public String checkoutPage(HttpSession session, Model model) {
        @SuppressWarnings("unchecked")
        List<CartItem> cart = (List<CartItem>) session.getAttribute("cart");
        
        // Nếu giỏ hàng trống thì đuổi về trang giỏ hàng
        if (cart == null || cart.isEmpty()) {
            return "redirect:/cart";
        }

        double total = 0;
        for (CartItem item : cart) {
            if (item.getPrice() != null) {
                total += item.getPrice() * item.getQuantity();
            }
        }
        
        model.addAttribute("title", "Thanh toán - Cửa Hàng Điện Máy");
        model.addAttribute("cartItems", cart);
        model.addAttribute("total", total);
        
        return "checkout";
    }

    // 7. XỬ LÝ ĐẶT HÀNG (Khi bấm nút Chốt Đơn)
    @PostMapping("/checkout/process")
    public String processCheckout(HttpSession session) {
        // Tương lai: Chúng ta sẽ lấy thông tin khách hàng và lưu dữ liệu Đơn hàng (Order) vào Database ở đây.
        
        // Hiện tại: Mô phỏng đặt hàng thành công bằng cách xóa giỏ hàng
        session.removeAttribute("cart");
        
        // Chuyển hướng sang trang báo thành công
        return "redirect:/checkout-success";
    }

    // 8. TRANG CẢM ƠN SAU KHI ĐẶT HÀNG
    @GetMapping("/checkout-success")
    public String checkoutSuccess(Model model) {
        model.addAttribute("title", "Đặt hàng thành công");
        return "checkout-success";
    }
}