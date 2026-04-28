package org.cuahangdienmay.cuahangdienmay_opensource.repository;

import org.cuahangdienmay.cuahangdienmay_opensource.model.Product;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import java.util.List;

@Repository
public interface ProductRepository extends JpaRepository<Product, Long> {
    
    // Phép màu của Spring JPA: Chỉ cần đặt tên hàm đúng chuẩn, nó tự dịch ra SQL!
    // Ví dụ: SELECT * FROM products WHERE is_flash_sale = true
    List<Product> findByIsFlashSaleTrue();
    
    List<Product> findByIsBestSellingTrue();
    
    List<Product> findByIsFeaturedTrue();
}