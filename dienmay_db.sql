INSERT INTO products (
    name, description, price, original_price, discount_percent, 
    stock_quantity, image_url, is_new, rating, reviews_count
) VALUES 
-- ==========================================
-- 1. ĐIỀU HÒA (dieuhoa_1 -> dieuhoa_10)
-- ==========================================
('Điều hòa Daikin Inverter 1 HP', 'Luồng gió Coanda, phin lọc Enzyme Blue diệt khuẩn.', 9500000, 11000000, 13, 50, 'http://localhost:3000/images/dieuhoa_1.jpg', true, 4.8, 120),
('Điều hòa Panasonic Inverter 1.5 HP', 'Công nghệ Nanoe-G lọc bụi mịn PM2.5, làm lạnh nhanh.', 12500000, 14000000, 10, 45, 'http://localhost:3000/images/dieuhoa_2.jpg', false, 4.9, 85),
('Điều hòa LG Inverter 1 HP', 'Dual Inverter tiết kiệm điện năng, dàn tản nhiệt mạ vàng.', 8000000, 9500000, 15, 60, 'http://localhost:3000/images/dieuhoa_3.jpg', true, 4.7, 210),
('Điều hòa Casper Inverter 1 HP', 'Cảm biến iFeel tự điều chỉnh nhiệt độ, tự làm sạch iClean.', 5500000, 7000000, 21, 100, 'http://localhost:3000/images/dieuhoa_4.jpg', false, 4.5, 340),
('Điều hòa Samsung Inverter 1.5 HP', 'Làm lạnh WindFree không gió buốt, bộ lọc Tri-Care.', 10500000, 12500000, 16, 30, 'http://localhost:3000/images/dieuhoa_5.jpg', true, 4.8, 90),
('Điều hòa Aqua Inverter 1 HP', 'Chế độ Eco tiết kiệm năng lượng, làm lạnh tức thì Turbo.', 6200000, 7500000, 17, 80, 'http://localhost:3000/images/dieuhoa_6.jpg', false, 4.6, 65),
('Điều hòa Toshiba Inverter 1 HP', 'Công nghệ Magic Coil chống bám bẩn, chế độ yên tĩnh.', 8200000, 9800000, 16, 40, 'http://localhost:3000/images/dieuhoa_7.jpg', false, 4.7, 110),
('Điều hòa Sharp Inverter 1.5 HP', 'Công nghệ Plasmacluster Ion diệt khuẩn, làm lạnh sâu.', 9800000, 11500000, 14, 25, 'http://localhost:3000/images/dieuhoa_8.jpg', true, 4.8, 75),
('Điều hòa Midea Inverter 1 HP', 'Thiết kế nhỏ gọn, cảnh báo rò rỉ ga an toàn.', 5000000, 6500000, 23, 120, 'http://localhost:3000/images/dieuhoa_9.jpg', false, 4.4, 200),
('Điều hòa Gree Inverter 1 HP', 'Cảm biến nhiệt độ I-Feel thông minh, màng lọc kháng khuẩn.', 6800000, 8500000, 20, 55, 'http://localhost:3000/images/dieuhoa_10.jpg', false, 4.6, 95),

-- ==========================================
-- 2. LAPTOP (laptop_1 -> laptop_10)
-- ==========================================
('Apple MacBook Air M1 256GB', 'Chip M1 mạnh mẽ, RAM 8GB, SSD 256GB, pin 18 giờ.', 18500000, 22000000, 15, 200, 'http://localhost:3000/images/laptop_1.jpg', false, 4.9, 950),
('Asus Zenbook 14 OLED', 'Màn hình OLED 2.8K 90Hz, Intel Core i5 Gen 12.', 21000000, 24500000, 14, 40, 'http://localhost:3000/images/laptop_2.jpg', true, 4.8, 112),
('Dell Inspiron 15 3520', 'Intel Core i5 1235U, RAM 8GB, SSD 512GB, màn hình 120Hz.', 14500000, 16900000, 14, 85, 'http://localhost:3000/images/laptop_3.jpg', false, 4.6, 320),
('HP Pavilion 14', 'Thiết kế vỏ kim loại sang trọng, Intel Core i5, RAM 8GB.', 15200000, 17500000, 13, 60, 'http://localhost:3000/images/laptop_4.jpg', false, 4.7, 180),
('Lenovo Legion 5 Gaming', 'Ryzen 7 5800H, RTX 3050Ti, Màn hình 165Hz chuẩn màu.', 25900000, 29000000, 10, 30, 'http://localhost:3000/images/laptop_5.jpg', true, 4.9, 210),
('Acer Nitro 5 Gaming', 'Core i5 12500H, RTX 3050, hệ thống tản nhiệt kép mát mẻ.', 19500000, 23000000, 15, 75, 'http://localhost:3000/images/laptop_6.jpg', false, 4.7, 450),
('MSI Bravo 15 Gaming', 'Ryzen 5 7535HS, RX 6550M, thiết kế hầm hố đậm chất game.', 16800000, 19500000, 13, 45, 'http://localhost:3000/images/laptop_7.jpg', true, 4.6, 88),
('Gigabyte Gaming G5', 'Core i5 11400H, RTX 3050, tần số quét 144Hz.', 17500000, 20500000, 14, 50, 'http://localhost:3000/images/laptop_8.jpg', false, 4.5, 120),
('LG Gram 14 Siêu Nhẹ', 'Trọng lượng chỉ 999g, độ bền chuẩn quân đội, pin cả ngày.', 24900000, 28900000, 13, 20, 'http://localhost:3000/images/laptop_9.jpg', true, 4.8, 65),
('Apple MacBook Pro M3', 'Chip M3 mới nhất, màn hình Liquid Retina XDR.', 38500000, 39990000, 3, 15, 'http://localhost:3000/images/laptop_10.jpg', true, 5.0, 42),

-- ==========================================
-- 3. MÁY GIẶT (maygiat_1 -> maygiat_10)
-- ==========================================
('Máy Giặt LG Inverter 10 kg', 'Truyền động trực tiếp AI DD bảo vệ sợi vải, giặt hơi nước.', 8900000, 11500000, 22, 60, 'http://localhost:3000/images/maygiat_1.jpg', true, 4.8, 310),
('Máy Giặt Samsung Inverter 9 kg', 'Công nghệ EcoBubble tạo bọt siêu mịn, cửa phụ AddWash.', 7500000, 9200000, 18, 85, 'http://localhost:3000/images/maygiat_2.jpg', false, 4.7, 240),
('Máy Giặt Electrolux Inverter 10 kg', 'Công nghệ UltraMix hòa tan bột giặt, diệt khuẩn Hygienic Care.', 10500000, 13000000, 19, 40, 'http://localhost:3000/images/maygiat_3.jpg', true, 4.9, 150),
('Máy Giặt Cửa Trên Toshiba 9 kg', 'Lồng giặt ngôi sao pha lê, mâm giặt kháng khuẩn Ag+.', 5100000, 6000000, 15, 100, 'http://localhost:3000/images/maygiat_4.jpg', false, 4.5, 180),
('Máy Giặt Panasonic Inverter 9.5 kg', 'Cảm biến Econavi tiết kiệm điện nước, giặt nước nóng StainMaster+.', 8200000, 10500000, 21, 55, 'http://localhost:3000/images/maygiat_5.jpg', false, 4.8, 95),
('Máy Giặt Aqua Inverter 8.5 kg', 'Lồng giặt Pillow bảo vệ quần áo, vòng đệm kháng khuẩn ABT.', 6000000, 7500000, 20, 70, 'http://localhost:3000/images/maygiat_6.jpg', false, 4.6, 112),
('Máy Giặt Beko Inverter 9 kg', 'Công nghệ sóng nước AquaWave, động cơ ProSmart êm ái.', 6800000, 8500000, 20, 30, 'http://localhost:3000/images/maygiat_7.jpg', true, 4.7, 45),
('Máy Giặt Casper Inverter 8.5 kg', 'Lồng giặt tinh thể pha lê, tính năng suy luận ảo Fuzzy Logic.', 4900000, 6500000, 24, 120, 'http://localhost:3000/images/maygiat_8.jpg', false, 4.5, 205),
('Máy Giặt Whirlpool Inverter 10.5 kg', 'Công nghệ giác quan thứ 6 6th Sense tự động định lượng nước.', 9500000, 11800000, 19, 25, 'http://localhost:3000/images/maygiat_9.jpg', true, 4.8, 60),
('Máy Giặt TCL Inverter 9 kg', 'Thiết kế lồng giặt tổ ong, vắt cực khô.', 5500000, 7000000, 21, 80, 'http://localhost:3000/images/maygiat_10.jpg', false, 4.4, 88),

-- ==========================================
-- 4. TIVI (tivi_1 -> tivi_10)
-- ==========================================
('Smart Tivi Samsung 4K 65 inch', 'Công nghệ PurColor màu sắc sống động, bộ xử lý Crystal 4K.', 14500000, 18000000, 19, 30, 'http://localhost:3000/images/tivi_1.jpg', true, 4.8, 210),
('Google Tivi Sony 4K 55 inch', 'Trí tuệ nhân tạo XR, âm thanh vòm Dolby Atmos đỉnh cao.', 16900000, 19000000, 11, 20, 'http://localhost:3000/images/tivi_2.jpg', false, 4.9, 145),
('Smart Tivi LG 4K 65 inch', 'Màn hình NanoCell màu sắc thuần khiết, Magic Remote thông minh.', 15200000, 18500000, 17, 35, 'http://localhost:3000/images/tivi_3.jpg', true, 4.8, 180),
('Google Tivi TCL 4K 55 inch', 'Công nghệ chấm lượng tử QLED, thiết kế tràn viền.', 8500000, 11000000, 22, 60, 'http://localhost:3000/images/tivi_4.jpg', false, 4.6, 250),
('Android Tivi Casper 4K 50 inch', 'Độ phân giải siêu nét, tìm kiếm giọng nói tiếng Việt 3 miền.', 6800000, 8500000, 20, 80, 'http://localhost:3000/images/tivi_5.jpg', false, 4.5, 315),
('Smart Tivi Hisense 4K 65 inch', 'Chế độ Game Mode tăng cường trải nghiệm, Dolby Vision.', 11500000, 14000000, 17, 25, 'http://localhost:3000/images/tivi_6.jpg', true, 4.7, 90),
('Google Tivi Coocaa 4K 55 inch', 'Màn hình chống ánh sáng xanh, điều khiển giọng nói rảnh tay.', 7200000, 9500000, 24, 70, 'http://localhost:3000/images/tivi_7.jpg', false, 4.5, 120),
('Smart Tivi Xiaomi 4K 55 inch', 'Khung kim loại nguyên khối, bộ nhớ 16GB mượt mà.', 8000000, 10500000, 23, 90, 'http://localhost:3000/images/tivi_8.jpg', true, 4.8, 410),
('Android Tivi Sharp 4K 60 inch', 'Chất lượng Nhật Bản, dải màu rộng Wide Color.', 10900000, 13500000, 19, 40, 'http://localhost:3000/images/tivi_9.jpg', false, 4.6, 85),
('Smart Tivi Toshiba 4K 55 inch', 'Công nghệ Regza Engine tối ưu hình ảnh, âm thanh Regza Power.', 9200000, 11500000, 20, 50, 'http://localhost:3000/images/tivi_10.jpg', false, 4.7, 105),

-- ==========================================
-- 5. TỦ LẠNH (tulanh_1 -> tulanh_10)
-- ==========================================
('Tủ Lạnh Panasonic Inverter 322 Lít', 'Ngăn đông mềm Prime Fresh+ giữ thịt cá tươi 7 ngày.', 10500000, 12000000, 12, 45, 'http://localhost:3000/images/tulanh_1.jpg', true, 4.9, 210),
('Tủ Lạnh Samsung Inverter 236 Lít', 'Công nghệ làm lạnh vòm, khay đá xoay di động tiện dụng.', 6200000, 7500000, 17, 65, 'http://localhost:3000/images/tulanh_2.jpg', false, 4.7, 340),
('Tủ Lạnh LG Inverter 393 Lít', 'Làm lạnh từ cửa tủ DoorCooling, ngăn lấy nước ngoài kháng khuẩn.', 12800000, 15000000, 14, 30, 'http://localhost:3000/images/tulanh_3.jpg', true, 4.8, 150),
('Tủ Lạnh Toshiba Inverter 253 Lít', 'Khử mùi diệt khuẩn Ag+ Bio, ngăn cấp đông mềm làm lạnh nhanh.', 7500000, 9000000, 16, 50, 'http://localhost:3000/images/tulanh_4.jpg', false, 4.6, 125),
('Tủ Lạnh Aqua Inverter 189 Lít', 'Nhỏ gọn tiết kiệm diện tích, khay kính chịu lực 100kg.', 4500000, 5500000, 18, 90, 'http://localhost:3000/images/tulanh_5.jpg', false, 4.5, 280),
('Tủ Lạnh Sharp Inverter 401 Lít', 'Hệ thống làm lạnh kép Hybrid Cooling, khử mùi phân tử bạc.', 11500000, 13500000, 14, 40, 'http://localhost:3000/images/tulanh_6.jpg', true, 4.8, 95),
('Tủ Lạnh Beko Inverter 320 Lít', 'Công nghệ ánh sáng xanh giả lập tự nhiên giúp rau củ tươi lâu.', 8900000, 11000000, 19, 35, 'http://localhost:3000/images/tulanh_7.jpg', false, 4.7, 88),
('Tủ Lạnh Electrolux Inverter 308 Lít', 'Ngăn rau củ TasteLock Auto tự bù ẩm, thiết kế châu Âu sang trọng.', 9500000, 11500000, 17, 45, 'http://localhost:3000/images/tulanh_8.jpg', true, 4.8, 112),
('Tủ Lạnh Casper Inverter 240 Lít', 'Hệ thống luồng khí lạnh đa chiều, không đóng tuyết.', 5500000, 6800000, 19, 80, 'http://localhost:3000/images/tulanh_9.jpg', false, 4.5, 170),
('Tủ Lạnh Side By Side Samsung 680 Lít', 'Dung tích siêu lớn, thiết kế mặt gương mờ sang trọng đẳng cấp.', 28500000, 32000000, 10, 15, 'http://localhost:3000/images/tulanh_10.jpg', true, 5.0, 65);



-- 1. Thêm cột slug vào bảng categories để khớp với mã bên Vue
ALTER TABLE categories ADD COLUMN slug VARCHAR(100) UNIQUE;

-- 2. Chèn 9 danh mục y hệt menu thiết kế của bạn
INSERT INTO categories (name, slug) VALUES 
('TiVi', 'tivi'),
('Tủ Lạnh', 'tulanh'),
('Máy Tính', 'maytinh'),
('Máy Giặt', 'maygiat'),
('Loa BlueTooth', 'loa'),
('Máy Lọc Nước', 'maylocnuoc'),
('Điện Thoại', 'dienthoai'),
('Đồng Hồ', 'dongho'),
('Khác', 'khac');

-- 3. "Phép thuật" tự động update category_id cho 50 sản phẩm cũ
UPDATE products SET category_id = (SELECT id FROM categories WHERE slug = 'tivi') WHERE name ILIKE '%Tivi%';
UPDATE products SET category_id = (SELECT id FROM categories WHERE slug = 'tulanh') WHERE name ILIKE '%Tủ lạnh%';
UPDATE products SET category_id = (SELECT id FROM categories WHERE slug = 'maygiat') WHERE name ILIKE '%Máy giặt%';
UPDATE products SET category_id = (SELECT id FROM categories WHERE slug = 'maytinh') WHERE name ILIKE '%MacBook%' OR name ILIKE '%Laptop%' OR name ILIKE '%Zenbook%' OR name ILIKE '%Inspiron%' OR name ILIKE '%Pavilion%' OR name ILIKE '%Legion%' OR name ILIKE '%Nitro%' OR name ILIKE '%Bravo%' OR name ILIKE '%Gaming%' OR name ILIKE '%Gram%';

-- (Vì Menu của bạn không có mục Điều hòa, tôi tạm gom 10 máy Điều hòa vào mục Khác nhé)
UPDATE products SET category_id = (SELECT id FROM categories WHERE slug = 'khac') WHERE name ILIKE '%Điều hòa%';





-- 1. Thêm danh mục "Điều hòa" vào bảng categories
INSERT INTO categories (name, slug) VALUES ('Điều hòa', 'dieuhoa');

-- 2. Chuyển 10 chiếc điều hòa từ mục "Khác" sang đúng mục "Điều hòa"
UPDATE products 
SET category_id = (SELECT id FROM categories WHERE slug = 'dieuhoa') 
WHERE name ILIKE '%Điều hòa%';

-- 3. (Tùy chọn) Xóa 2 danh mục thừa là "Điện Thoại" và "Đồng Hồ" cho sạch Database
DELETE FROM categories WHERE slug IN ('dienthoai', 'dongho');





-- 1. Cập nhật ảnh mới cho tivi_6
UPDATE products 
SET image_url = 'http://localhost:3000/images/tivi_6.jpg' 
WHERE image_url LIKE '%tivi_6.jpg';

-- 2. Cập nhật ảnh mới cho maygiat_10
UPDATE products 
SET image_url = 'http://localhost:3000/images/maygiat_10.jpg' 
WHERE image_url LIKE '%maygiat_10.jpg';

-- 3. Cập nhật ảnh mới cho maygiat_8
UPDATE products 
SET image_url = 'http://localhost:3000/images/maygiat_8.jpg' 
WHERE image_url LIKE '%maygiat_8.jpg';

-- 4. Cập nhật ảnh mới cho tulanh_3
UPDATE products 
SET image_url = 'http://localhost:3000/images/tulanh_3.jpg' 
WHERE image_url LIKE '%tulanh_3.jpg';

-- 5. Cập nhật ảnh mới cho dieuhoa_5
UPDATE products 
SET image_url = 'http://localhost:3000/images/dieuhoa_5.jpg' 
WHERE image_url LIKE '%dieuhoa_5.jpg';





UPDATE products 
SET image_url = 'http://localhost:3000/images/tivi_6_new.jpg' 
WHERE name ILIKE '%Smart Tivi Hisense 4K 65 inch%';

-- 1. Cập nhật ảnh cho maygiat_10
UPDATE products 
SET image_url = 'http://localhost:3000/images/maygiat_10_new.jpg' 
WHERE name ILIKE '%Máy Giặt TCL Inverter 9 kg%';

-- 2. Cập nhật ảnh cho maygiat_8
UPDATE products 
SET image_url = 'http://localhost:3000/images/maygiat_8_new.jpg' 
WHERE name ILIKE '%Máy Giặt Casper Inverter 8.5 kg%';

-- 3. Cập nhật ảnh cho tulanh_3
UPDATE products 
SET image_url = 'http://localhost:3000/images/tulanh_3_new.jpg' 
WHERE name ILIKE '%Tủ Lạnh LG Inverter 393 Lít%';

-- 4. Cập nhật ảnh cho dieuhoa_5
UPDATE products 
SET image_url = 'http://localhost:3000/images/dieuhoa_5_new.jpg' 
WHERE name ILIKE '%Điều hòa Samsung Inverter 1.5 HP%';


select * from products