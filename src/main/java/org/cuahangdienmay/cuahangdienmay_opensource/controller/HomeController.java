package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import org.cuahangdienmay.cuahangdienmay_opensource.repository.ProductRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;

import java.util.List;
import java.util.Map;

@Controller
public class HomeController {
        @Autowired
        private ProductRepository productRepository;

        @GetMapping("/")
        public String home(Model model) {
                model.addAttribute("title", "Cửa Hàng Điện Máy - Mua sắm thông minh");

                // Quét DB thật và truyền ra View
                model.addAttribute("flashSales", productRepository.findByIsFlashSaleTrue());
                model.addAttribute("bestSelling", productRepository.findByIsBestSellingTrue());
                model.addAttribute("featuredProducts", productRepository.findByIsFeaturedTrue());
                
                return "index";
        };
        
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