-- =========================================
-- PHẦN 1: TẠO BẢNG (An toàn, không lỗi nếu bảng đã có)
-- =========================================
CREATE TABLE IF NOT EXISTS product_specifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    spec_key VARCHAR(100) NOT NULL,
    spec_value TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS product_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    user_name VARCHAR(100) NOT NULL,
    rating INTEGER NOT NULL CHECK (rating >= 1 AND rating <= 5),
    content TEXT,
    is_verified BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- =========================================
-- PHẦN 2: BƠM DỮ LIỆU VÀO TIVI SONY
-- =========================================
DO $$ 
DECLARE target_id UUID;
BEGIN
    -- Tìm ID của Tivi Sony 55 inch
    SELECT id INTO target_id FROM products 
    WHERE name ILIKE '%Sony%55%inch%' LIMIT 1;

    -- Nếu tìm thấy mới bắt đầu bơm dữ liệu
    IF target_id IS NOT NULL THEN
        
        -- Dọn dẹp dữ liệu cũ (nếu có)
        DELETE FROM product_specifications WHERE product_id = target_id;
        DELETE FROM product_reviews WHERE product_id = target_id;

        -- Bơm Thông số kỹ thuật
        INSERT INTO product_specifications (product_id, spec_key, spec_value) VALUES 
        (target_id, 'Thương hiệu', 'Sony (Nhật Bản)'),
        (target_id, 'Năm ra mắt', '2023'),
        (target_id, 'Kích thước màn hình', '55 inch'),
        (target_id, 'Độ phân giải', '4K (Ultra HD)'),
        (target_id, 'Hệ điều hành', 'Google TV'),
        (target_id, 'Tổng công suất loa', '20W');

        -- Bơm Đánh giá
        INSERT INTO product_reviews (product_id, user_name, rating, content) VALUES 
        (target_id, 'Nguyễn Văn Hùng', 5, 'Hình ảnh 4K cực kỳ sắc nét, màu sắc rực rỡ đúng chất Sony. Thiết kế tràn viền nhìn rất sang trọng.'),
        (target_id, 'Trần Thị Mai', 5, 'Lắp đặt nhanh chóng. Giao diện Google TV rất dễ sử dụng, điều khiển giọng nói tiếng Việt nhận diện tốt.'),
        (target_id, 'Lê Minh Tuấn', 4, 'Sản phẩm tuyệt vời, tuy nhiên phần loa nghe ở mức chấp nhận được.');
        
        RAISE NOTICE 'Đã cập nhật dữ liệu thật cho sản phẩm Sony thành công!';
    ELSE
        RAISE NOTICE 'Không tìm thấy sản phẩm Sony 55 inch trong Database.';
    END IF;
END $$;