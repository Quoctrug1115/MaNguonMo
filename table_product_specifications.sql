-- 1. Tạo bảng biến thể sản phẩm
CREATE TABLE product_variants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    color_name VARCHAR(50), -- Ví dụ: "Đen không gian"
    color_hex VARCHAR(7),   -- Ví dụ: "#000000"
    stock INT DEFAULT 0,    -- Tồn kho riêng cho màu này
    price_override DECIMAL(12,2), -- Giá riêng (nếu màu này đắt hơn, có thể để NULL nếu dùng giá chung)
    created_at TIMESTAMP DEFAULT NOW()
);

-- 2. Thêm cột Tổng tồn kho vào bảng products
ALTER TABLE products ADD COLUMN total_stock INT DEFAULT 0;

-- 3. (Tùy chọn) Xóa cột stock cũ ở bảng products nếu bạn muốn quản lý hoàn toàn qua variants
-- ALTER TABLE products DROP COLUMN total_stock;


-- 1. Thêm 1 sản phẩm gốc (Đã loại bỏ cột total_stock)
INSERT INTO products (id, name, description, price, image_url)
VALUES (
    '11111111-1111-1111-1111-111111111111', 
    'Apple Watch Series 4', 
    'Đồng hồ thông minh', 
    690.00, 
    'https://cdn.pixabay.com/photo/2014/12/08/14/23/smart-watch-560941_1280.jpg'
);

-- 2. Thêm 3 biến thể màu sắc cho sản phẩm trên (Giữ nguyên)
INSERT INTO product_variants (product_id, color_name, color_hex, stock) VALUES
('11111111-1111-1111-1111-111111111111', 'Đen', '#000000', 20),
('11111111-1111-1111-1111-111111111111', 'Xám', '#808080', 30),
('11111111-1111-1111-1111-111111111111', 'Hồng', '#FFC0CB', 13);


select * from products


-- Thêm cột hình ảnh riêng cho từng màu
ALTER TABLE product_variants ADD COLUMN image_url TEXT;

-- (Tùy chọn) Cập nhật thử 1 ảnh cho màu Đen để test
UPDATE product_variants 
SET image_url = 'https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=500' 
WHERE color_name = 'Đen';


ALTER TABLE product_specifications RENAME COLUMN spec_key TO spec_name;
ALTER TABLE product_specifications RENAME COLUMN spec_value TO spec_value;


select * from product_variants