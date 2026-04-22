package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;

import java.util.List;
import java.util.Map;

@Controller
public class HomeController {

    @GetMapping("/")
    public String home(Model model) {
        // Tạo danh sách sản phẩm Flash Sale mẫu
        List<Map<String, Object>> flashSales = List.of(
                Map.of("id", 1, "name", "Tay cầm chơi game Havic G92", "price", 120, "oldPrice", 160, "discount", 40, "rating", 88, "img", "/images/gamepad.png"),
                Map.of("id", 2, "name", "Bàn phím cơ AKKO 3068", "price", 960, "oldPrice", 1160, "discount", 35, "rating", 75, "img", "/images/keyboard.png"),
                Map.of("id", 3, "name", "Màn hình Gaming IPS 24 inch", "price", 370, "oldPrice", 400, "discount", 30, "rating", 99, "img", "/images/monitor.png"),
                Map.of("id", 4, "name", "Loa Bluetooth JBL Flip 6", "price", 160, "oldPrice", 170, "discount", 25, "rating", 65, "img", "/images/speaker.png"),
                Map.of("id", 5, "name", "Máy ảnh Canon EOS R5", "price", 2500, "oldPrice", 3000, "discount", 15, "rating", 120, "img", "/images/camera.png")
        );

        // Danh sách Sản Phẩm Bán Chạy
        List<Map<String, Object>> bestSelling = List.of(
                Map.of("id", 6, "name", "Áo khoác The North Face", "price", 260, "oldPrice", 360, "rating", 65, "img", "/images/jacket.png"),
                Map.of("id", 7, "name", "Túi xách Gucci", "price", 960, "oldPrice", 1160, "rating", 65, "img", "/images/bag.png"),
                Map.of("id", 8, "name", "Tản nhiệt CPU RGB", "price", 160, "oldPrice", 170, "rating", 65, "img", "/images/cooler.png"),
                Map.of("id", 9, "name", "Kệ sách gỗ sồi", "price", 360, "rating", 65, "img", "/images/shelf.png") // Sản phẩm này không có giá cũ
        );
        // Danh sách Sản Phẩm Nổi Bật (Explore Our Products)
        List<Map<String, Object>> featuredProducts = List.of(
                Map.of("id", 10, "name", "Chó Robot Đồ Chơi", "price", 120, "rating", 88, "img", "/images/dog.png"),
                Map.of("id", 11, "name", "Máy ảnh DSLR Canon", "price", 360, "rating", 75, "img", "/images/camera2.png"),
                Map.of("id", 12, "name", "Laptop Gaming ASUS", "price", 700, "rating", 99, "img", "/images/laptop.png"),
                Map.of("id", 13, "name", "Kem dưỡng da Curology", "price", 500, "rating", 65, "img", "/images/curology.png"),
                Map.of("id", 14, "name", "Xe điện trẻ em", "price", 960, "isNew", true, "rating", 65, "img", "/images/car.png"), // Có nhãn Mới
                Map.of("id", 15, "name", "Giày thể thao Zoom", "price", 1160, "rating", 35, "img", "/images/shoes.png"),
                Map.of("id", 16, "name", "Tay cầm DualShock", "price", 660, "isNew", true, "rating", 55, "img", "/images/gamepad2.png"), // Có nhãn Mới
                Map.of("id", 17, "name", "Áo khoác Satin", "price", 660, "rating", 55, "img", "/images/jacket2.png")
        );

        model.addAttribute("featuredProducts", featuredProducts);
        model.addAttribute("title", "Cửa Hàng Điện Máy...");
        model.addAttribute("bestSelling", bestSelling);
        model.addAttribute("flashSales", flashSales);
        return "index";
        }
        @GetMapping("/wishlist")
        public String wishlist(Model model) {
                model.addAttribute("title", "Danh sách yêu thích - Cửa Hàng Điện Máy");
                
                // Mock data cho Wishlist
                List<Map<String, Object>> wishlistItems = List.of(
                Map.of("id", 1, "name", "Túi xách Gucci", "price", 960, "oldPrice", 1160, "discount", 35, "img", "/images/bag.png"),
                Map.of("id", 2, "name", "Loa Bluetooth JBL", "price", 160, "img", "/images/speaker.png"),
                Map.of("id", 3, "name", "Tay cầm Havic HV G-92", "price", 120, "img", "/images/gamepad.png")
                );
                model.addAttribute("wishlistItems", wishlistItems);
                return "wishlist"; 
        }

        @GetMapping("/404")
        public String error404(Model model) {
                model.addAttribute("title", "Không tìm thấy trang - 404");
                return "404"; 
        }
}