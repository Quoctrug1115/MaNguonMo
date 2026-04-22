package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import java.util.List;
import java.util.Map;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;

@Controller
public class ProductController {

    // Đường dẫn có chứa biến {id} để biết khách đang xem sản phẩm nào
    @GetMapping("/product/{id}")
    public String productDetail(@PathVariable("id") int id, Model model) {
        // Giả lập dữ liệu sản phẩm (Thực tế sẽ gọi từ Database lên)
        model.addAttribute("title", "Tay cầm Havic HV G-92 - Cửa Hàng Điện Máy");
        model.addAttribute("productName", "Havic HV G-92 Gamepad");
        model.addAttribute("price", 192.00);
        model.addAttribute("rating", 150);
        model.addAttribute("description", "PlayStation 5 Controller Skin. Chất lượng cao, dán dễ dàng, không để lại keo khi bóc. Trải nghiệm cầm nắm cực kỳ thoải mái.");
        
        return "product-detail"; 
    }
    @GetMapping("/products")
    public String allProducts(Model model) {
        model.addAttribute("title", "Tất cả sản phẩm - Cửa Hàng Điện Máy");
        
        // Tạo danh sách 9 sản phẩm mẫu để test giao diện phân trang
        List<Map<String, Object>> products = List.of(
            Map.of("id", 1, "name", "Tay cầm Havic HV G-92", "price", 120, "oldPrice", 160, "rating", 88, "img", "/images/gamepad.png"),
            Map.of("id", 2, "name", "Bàn phím cơ AKKO 3068", "price", 960, "rating", 75, "img", "/images/keyboard.png"),
            Map.of("id", 3, "name", "Màn hình Gaming IPS", "price", 370, "oldPrice", 400, "rating", 99, "img", "/images/monitor.png"),
            Map.of("id", 4, "name", "Loa Bluetooth JBL", "price", 160, "rating", 65, "img", "/images/speaker.png"),
            Map.of("id", 5, "name", "Máy ảnh Canon EOS", "price", 2500, "oldPrice", 3000, "rating", 120, "img", "/images/camera.png"),
            Map.of("id", 6, "name", "Laptop Gaming ASUS", "price", 1500, "rating", 200, "img", "/images/laptop.png"),
            Map.of("id", 7, "name", "Chuột Logitech G Pro", "price", 130, "rating", 450, "img", "/images/mouse.png"),
            Map.of("id", 8, "name", "Tai nghe Sony WH-1000XM5", "price", 350, "oldPrice", 400, "rating", 320, "img", "/images/headphone.png"),
            Map.of("id", 9, "name", "Micro thu âm Rode", "price", 180, "rating", 55, "img", "/images/micro.png")
        );
        
        model.addAttribute("products", products);
        return "products"; // Trả về file products.html
    }
}