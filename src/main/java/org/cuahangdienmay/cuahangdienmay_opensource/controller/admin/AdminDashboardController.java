package org.cuahangdienmay.cuahangdienmay_opensource.controller.admin;

import java.util.List;
import java.util.Map;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;

@Controller
@RequestMapping("/admin") // Mọi đường dẫn trong này đều bắt đầu bằng /admin
public class AdminDashboardController {

    @GetMapping({"", "/dashboard"})
    public String dashboard(Model model) {
        model.addAttribute("title", "Dashboard - Admin Quản Trị");
        return "admin/dashboard"; // Trỏ vào thư mục templates/admin/
    }
    @GetMapping("/products")
    public String products(Model model) {
        model.addAttribute("title", "Quản lý Sản Phẩm - Admin");
        // Giả lập dữ liệu Sản phẩm
        model.addAttribute("products", List.of(
            Map.of("id", "PRO-01", "name", "Apple Watch Series 8", "category", "Đồng hồ", "price", 399, "stock", 45, "img", "https://placehold.co/40x40"),
            Map.of("id", "PRO-02", "name", "MacBook Pro M2", "category", "Laptop", "price", 1299, "stock", 12, "img", "https://placehold.co/40x40"),
            Map.of("id", "PRO-03", "name", "Tai nghe AirPods Pro", "category", "Phụ kiện", "price", 249, "stock", 0, "img", "https://placehold.co/40x40") // Hết hàng
        ));
        return "admin/admin-products"; 
    }

    @GetMapping("/orders")
    public String orders(Model model) {
        model.addAttribute("title", "Quản lý Đơn Hàng - Admin");
        // Giả lập dữ liệu Đơn hàng
        model.addAttribute("orders", List.of(
            Map.of("id", "ORD-1001", "customer", "Nguyễn Văn A", "date", "22/04/2026", "total", 1698, "status", "Delivered"),
            Map.of("id", "ORD-1002", "customer", "Trần Thị B", "date", "21/04/2026", "total", 249, "status", "Processing"),
            Map.of("id", "ORD-1003", "customer", "Lê Văn C", "date", "20/04/2026", "total", 399, "status", "Pending"),
            Map.of("id", "ORD-1004", "customer", "Hoàng Anh D", "date", "19/04/2026", "total", 1299, "status", "Cancelled")
        ));
        return "admin/admin-orders"; 
    }
}