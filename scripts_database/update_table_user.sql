-- 1. Cho phép mật khẩu được bỏ trống (Dành cho khách đăng nhập bằng Google)
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;

-- 2. Thêm cột lưu mã định danh của Google (để nhận diện khách hàng ở lần đăng nhập sau)
ALTER TABLE users ADD COLUMN IF NOT EXISTS google_id VARCHAR(255) UNIQUE;

-- 3. Thêm cột lưu link ảnh đại diện (Lấy thẳng ảnh từ Gmail sang cho đẹp)
ALTER TABLE users ADD COLUMN IF NOT EXISTS avatar_url TEXT;


select * from users



select * from products


