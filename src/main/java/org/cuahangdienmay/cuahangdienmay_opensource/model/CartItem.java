package org.cuahangdienmay.cuahangdienmay_opensource.model;

public class CartItem {
    private Long productId;
    private String name;
    private Double price;
    private int quantity;
    private String img;

    // Constructor rỗng (Rất quan trọng để tránh lỗi khi Spring khởi tạo đối tượng)
    public CartItem() {
    }

    // Constructor có tham số
    public CartItem(Long productId, String name, Double price, int quantity, String img) {
        this.productId = productId;
        this.name = name;
        this.price = price;
        this.quantity = quantity;
        this.img = img;
    }

    // ==========================================
    // BẮT BUỘC PHẢI CÓ CÁC HÀM NÀY ĐỂ HTML ĐỌC ĐƯỢC
    // ==========================================
    
    public Long getProductId() {
        return productId;
    }

    public void setProductId(Long productId) {
        this.productId = productId;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public Double getPrice() {
        return price;
    }

    public void setPrice(Double price) {
        this.price = price;
    }

    public int getQuantity() {
        return quantity;
    }

    public void setQuantity(int quantity) {
        this.quantity = quantity;
    }

    public String getImg() {
        return img;
    }

    public void setImg(String img) {
        this.img = img;
    }
}