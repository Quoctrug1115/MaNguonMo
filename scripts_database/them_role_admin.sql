UPDATE users 
SET role = 'admin' 
WHERE email = 'lqtrung11152005@gmail.com';

select * from products


ALTER TABLE cart_items 
ADD CONSTRAINT unique_user_product UNIQUE (user_id, product_id);