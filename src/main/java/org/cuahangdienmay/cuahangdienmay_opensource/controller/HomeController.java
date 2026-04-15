package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;

@Controller
public class HomeController {

    @GetMapping("/")
    public String home(Model model) {
        // Gửi thử một dòng dữ liệu từ Java xuống HTML
        model.addAttribute("title", "Cửa Hàng Điện Máy - Mua sắm thông minh");
        return "index"; // Trả về file index.html trong thư mục templates
    }
}