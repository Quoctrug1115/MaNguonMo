package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import java.util.List;
import java.util.Map;

@Controller
public class CartController {

    @GetMapping("/cart")
    public String viewCart(Model model) {
        model.addAttribute("title", "Giỏ hàng của bạn - Cửa Hàng Điện Máy");

        // Giả lập 2 sản phẩm đang có trong giỏ hàng (giống hình Figma)
        List<Map<String, Object>> cartItems = List.of(
            Map.of("id", 1, "name", "Màn hình LCD Gaming", "price", 650, "quantity", 1, "img", "/images/monitor.png"),
            Map.of("id", 2, "name", "Tay cầm Havic HV G-92", "price", 550, "quantity", 2, "img", "/images/gamepad.png")
        );

        model.addAttribute("cartItems", cartItems);
        
        // Tính tổng tiền tạm tính
        double subtotal = cartItems.stream()
            .mapToDouble(item -> (double) (Integer) item.get("price") * (Integer) item.get("quantity"))
            .sum();
            
        model.addAttribute("subtotal", subtotal);

        return "cart"; 
    }
    @GetMapping("/checkout")
    public String checkout(Model model) {
        model.addAttribute("title", "Thanh toán - Cửa Hàng Điện Máy");

        // Lấy lại danh sách sản phẩm từ Giỏ hàng
        List<Map<String, Object>> checkoutItems = List.of(
            Map.of("name", "Màn hình LCD Gaming", "price", 650, "img", "/images/monitor.png"),
            Map.of("name", "Tay cầm Havic HV G-92", "price", 1100, "img", "/images/gamepad.png") // Giả sử mua 2 cái nên giá 1100
        );

        model.addAttribute("checkoutItems", checkoutItems);
        model.addAttribute("total", 1750); // Tổng thanh toán

        return "checkout"; 
    }
}