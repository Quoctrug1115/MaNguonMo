package org.cuahangdienmay.cuahangdienmay_opensource.controller.admin;

import org.cuahangdienmay.cuahangdienmay_opensource.model.Product;
import org.cuahangdienmay.cuahangdienmay_opensource.repository.ProductRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;

import java.util.List;
import java.util.Map;
@Controller
@RequestMapping("/admin") // Mọi đường dẫn trong này đều bắt đầu bằng /admin
public class AdminDashboardController {
    @Autowired
    private ProductRepository productRepository;

    @GetMapping({"", "/dashboard"})
    public String dashboard(Model model) {
        model.addAttribute("title", "Dashboard - Admin");
        return "admin/dashboard"; 
    }
    @GetMapping("/products")
    public String products(Model model) {
        model.addAttribute("title", "Quản lý Sản Phẩm - Admin");
        // Quét dữ liệu thật từ Database
        model.addAttribute("products", productRepository.findAll());
        return "admin/admin-products"; 
    }

    // CHỈ CÓ 1 HÀM GET NÀY CHO TRANG THÊM SẢN PHẨM
    @GetMapping("/product-add")
    public String addProductForm(Model model) {
        model.addAttribute("title", "Thêm Sản Phẩm Mới - Admin");
        model.addAttribute("product", new Product()); 
        return "admin/admin-product-add"; 
    }

    @PostMapping("/product-add")
    public String saveProduct(@ModelAttribute("product") Product product) {
        if (product.getImg() == null || product.getImg().isEmpty()) {
            product.setImg("https://placehold.co/400x400?text=No+Image");
        }
        productRepository.save(product);
        return "redirect:/admin/products"; 
    }

    @GetMapping("/product-delete/{id}")
    public String deleteProduct(@PathVariable("id") Long id) {
        productRepository.deleteById(id);
        return "redirect:/admin/products";
    }

    @GetMapping("/product-edit/{id}")
    public String editProductForm(@PathVariable("id") Long id, Model model) {
        // Tìm sản phẩm trong Database
        Product product = productRepository.findById(id).orElse(new Product());
        
        model.addAttribute("title", "Cập nhật Sản Phẩm - Admin");
        model.addAttribute("product", product); // Nhét dữ liệu cũ vào biến product
        
        // Tái sử dụng lại form Thêm Mới để hiển thị!
        return "admin/admin-product-add";
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

    @GetMapping("/customers")
    public String customers(Model model) {
        model.addAttribute("title", "Quản lý Khách Hàng - Admin");
        // Giả lập dữ liệu Khách hàng
        model.addAttribute("customers", List.of(
            Map.of("id", "CUS-001", "name", "Nguyễn Văn A", "email", "nva@gmail.com", "phone", "0901234567", "spent", 1500, "status", "Active"),
            Map.of("id", "CUS-002", "name", "Trần Thị B", "email", "ttb@gmail.com", "phone", "0987654321", "spent", 320, "status", "Active"),
            Map.of("id", "CUS-003", "name", "Lê Hacker", "email", "hacker@darkweb.com", "phone", "0123456789", "spent", 0, "status", "Blocked")
        ));
        return "admin/admin-customers"; 
    }
}