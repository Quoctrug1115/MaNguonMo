package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import org.springframework.ui.Model;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;

@Controller
public class AuthController {
    @GetMapping("/login")
    public String login(Model model) {
        model.addAttribute("title", "Đăng nhập - Cửa Hàng Điện Máy");
        return "login"; // Trả về file login.html
    }

    @GetMapping("/register")
    public String register(Model model) {
        model.addAttribute("title", "Đăng ký - Cửa Hàng Điện Máy");
        return "register"; // Trả về file register.html
    }
    @GetMapping("/forgot-password")
    public String forgotPassword(Model model) {
        model.addAttribute("title", "Quên mật khẩu - Cửa Hàng Điện Máy");
        return "forgot-password"; 
    }
}
