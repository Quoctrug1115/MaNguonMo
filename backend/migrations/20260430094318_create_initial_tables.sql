-- Bảng Users (Người dùng)
CREATE TABLE users (
                       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                       full_name VARCHAR(100) NOT NULL,
                       email VARCHAR(255) UNIQUE NOT NULL,
                       password_hash VARCHAR(255) NOT NULL,
                       phone VARCHAR(20),
                       address TEXT,
                       role VARCHAR(20) DEFAULT 'customer', -- 'customer' hoặc 'admin'
                       created_at TIMESTAMPTZ DEFAULT NOW(),
                       updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Bảng Categories (Danh mục sản phẩm)
CREATE TABLE categories (
                            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                            name VARCHAR(100) UNIQUE NOT NULL,
                            description TEXT,
                            created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Bảng Products (Sản phẩm)
CREATE TABLE products (
                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                          category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
                          name VARCHAR(255) NOT NULL,
                          description TEXT,
                          price BIGINT NOT NULL, -- Dùng BIGINT lưu giá tiền VNĐ để tránh lỗi làm tròn số thực
                          original_price BIGINT,
                          discount_percent INT DEFAULT 0,
                          stock_quantity INT NOT NULL DEFAULT 0,
                          image_url TEXT,
                          is_new BOOLEAN DEFAULT false,
                          rating FLOAT DEFAULT 0.0,
                          reviews_count INT DEFAULT 0,
                          created_at TIMESTAMPTZ DEFAULT NOW(),
                          updated_at TIMESTAMPTZ DEFAULT NOW()
);