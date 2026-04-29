package org.cuahangdienmay.cuahangdienmay_opensource.repository;

import org.cuahangdienmay.cuahangdienmay_opensource.model.User;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface UserRepository extends JpaRepository<User, Long> {
    // Hàm tự chế: Giúp tìm tài khoản xem đã tồn tại chưa
    User findByUsername(String username);
}