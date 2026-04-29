package org.cuahangdienmay.cuahangdienmay_opensource.controller;

import org.cuahangdienmay.cuahangdienmay_opensource.model.User;
import org.cuahangdienmay.cuahangdienmay_opensource.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;

import jakarta.servlet.http.HttpSession;

@Controller
public class AuthController {

    @Autowired
    private UserRepository userRepository;

    // 1. Mở trang Đăng ký
    @GetMapping("/register")
    public String registerForm(Model model) {
        model.addAttribute("title", "Đăng ký tài khoản");
        model.addAttribute("user", new User()); // Gửi object rỗng xuống form
        return "register";
    }

    // 2. Xử lý khi bấm nút Đăng ký
    @PostMapping("/register")
    public String processRegister(@ModelAttribute("user") User user, Model model) {
        // Kiểm tra xem username đã có ai dùng chưa
        User existingUser = userRepository.findByUsername(user.getUsername());
        if (existingUser != null) {
            model.addAttribute("error", "Tên đăng nhập đã tồn tại! Vui lòng chọn tên khác.");
            return "register";
        }

        // Set quyền mặc định là Khách hàng (USER)
        user.setRole("USER");
        
        // Lưu vào Database
        userRepository.save(user);
        
        // Đăng ký thành công thì chuyển hướng về trang Đăng nhập
        return "redirect:/login?success";
    }

    // 3. Mở trang Đăng nhập (Chúng ta sẽ làm chức năng Login sau)
    @GetMapping("/login")
    public String loginForm(Model model) {
        model.addAttribute("title", "Đăng nhập");
        return "login";
    }

    // 4. Xử lý dữ liệu khi bấm nút Đăng nhập
    @PostMapping("/login")
    public String processLogin(@RequestParam("username") String username,
                               @RequestParam("password") String password,
                               HttpSession session,
                               Model model) {
        
        // 1. Tìm tài khoản trong Database
        User user = userRepository.findByUsername(username);

        // 2. Kiểm tra xem tài khoản có tồn tại và mật khẩu có khớp không
        // (Lưu ý: Hiện tại đang so sánh mật khẩu trần. Ở dự án thực tế, Senior sẽ dùng BCrypt để mã hóa)
        if (user != null && user.getPassword().equals(password)) {
            
            // 3. Đăng nhập thành công -> Lưu thông tin User vào Session (Bộ nhớ phiên)
            session.setAttribute("loggedInUser", user);
            
            // 4. Chuyển hướng về trang chủ
            return "redirect:/"; 
        } else {
            // Đăng nhập thất bại -> Ở lại trang login và hiện lỗi
            model.addAttribute("error", "Tên đăng nhập hoặc mật khẩu không chính xác!");
            return "login";
        }
    }

    // 5. Xử lý Đăng xuất
    @GetMapping("/logout")
    public String logout(HttpSession session) {
        // Xóa thông tin user khỏi Session
        session.removeAttribute("loggedInUser");
        return "redirect:/login";
    }

    // 6. TRANG HỒ SƠ CÁ NHÂN (PROFILE)
    @GetMapping("/profile")
    public String viewProfile(HttpSession session, Model model) {
        // Kiểm tra xem đã đăng nhập chưa
        User loggedInUser = (User) session.getAttribute("loggedInUser");
        
        // Nếu chưa đăng nhập mà cố tình vào Profile -> Đuổi về trang Login
        if (loggedInUser == null) {
            return "redirect:/login";
        }
        
        // Nếu đã đăng nhập -> Mở trang profile
        model.addAttribute("title", "Hồ sơ của " + loggedInUser.getFullName());
        model.addAttribute("user", loggedInUser);
        return "profile";
    }
}