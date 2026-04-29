package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import jakarta.servlet.http.HttpSession;
import org.cuahangdienmay.cuahangdienmay_opensource.model.CartItem;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ModelAttribute;

import java.util.List;

@ControllerAdvice
public class GlobalControllerAdvice {

    // Hàm này sẽ tự động chạy ở mọi Request.
    // Biến "cartItemCount" sẽ được nhét sẵn vào mọi file HTML.
    @ModelAttribute("cartItemCount")
    public int getCartItemCount(HttpSession session) {
        @SuppressWarnings("unchecked")
        List<CartItem> cart = (List<CartItem>) session.getAttribute("cart");
        
        if (cart == null || cart.isEmpty()) {
            return 0;
        }
        
        // Tính tổng số lượng của tất cả các món trong giỏ
        return cart.stream().mapToInt(CartItem::getQuantity).sum();
    }
}