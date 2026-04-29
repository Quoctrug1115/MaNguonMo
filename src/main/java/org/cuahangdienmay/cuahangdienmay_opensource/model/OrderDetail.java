package org.cuahangdienmay.cuahangdienmay_opensource.model;

import jakarta.persistence.*;

@Entity
@Table(name = "order_details")
public class OrderDetail {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id;

    // Mối quan hệ Nhiều-1: Nhiều Chi tiết thuộc về 1 Đơn hàng
    @ManyToOne
    @JoinColumn(name = "order_id")
    private Order order;

    // Mối quan hệ Nhiều-1: 1 Chi tiết chứa 1 Sản phẩm
    @ManyToOne
    @JoinColumn(name = "product_id")
    private Product product;

    private int quantity; // Số lượng mua
    private Double price; // Giá tiền TẠI THỜI ĐIỂM MUA (Để lỡ sau này SP tăng giá thì đơn hàng cũ không bị đổi giá)
    public Long getId() {
        return id;
    }
    public void setId(Long id) {
        this.id = id;
    }
    public Order getOrder() {
        return order;
    }
    public void setOrder(Order order) {
        this.order = order;
    }
    public Product getProduct() {
        return product;
    }
    public void setProduct(Product product) {
        this.product = product;
    }
    public int getQuantity() {
        return quantity;
    }
    public void setQuantity(int quantity) {
        this.quantity = quantity;
    }
    public Double getPrice() {
        return price;
    }
    public void setPrice(Double price) {
        this.price = price;
    }

    
}