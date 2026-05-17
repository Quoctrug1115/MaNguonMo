-- 1. Bảng Giỏ Hàng (Lưu nháp các món đồ khách muốn mua)
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL DEFAULT 1 CHECK (quantity > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Cực kỳ quan trọng: 1 user chỉ có 1 dòng duy nhất cho 1 sản phẩm cụ thể trong giỏ
    UNIQUE(user_id, product_id) 
);

-- 2. Bảng Đơn Hàng (Thông tin tổng quát của hóa đơn)
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    total_price NUMERIC(15, 0) NOT NULL, -- Dùng NUMERIC để lưu tiền tệ chính xác nhất
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending (chờ duyệt), processing (đang xử lý), shipped (đang giao), delivered (đã giao), cancelled (đã hủy)
    shipping_address TEXT NOT NULL,
    phone_number VARCHAR(20) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 3. Bảng Chi Tiết Đơn Hàng (Các món đồ cụ thể nằm trong hóa đơn)
CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    price_at_purchase NUMERIC(15, 0) NOT NULL, -- Giá chốt tại thời điểm bấm đặt hàng
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);



select * from cart_items