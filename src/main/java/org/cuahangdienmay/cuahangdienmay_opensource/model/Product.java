package org.cuahangdienmay.cuahangdienmay_opensource.model;

import jakarta.persistence.*;

@Entity
@Table(name = "products")
public class Product {

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY) // ID tự động tăng
    private Long id;

    @Column(nullable = false)
    private String name;

    @Column(nullable = false)
    private Double price;

    private Double oldPrice; // Dùng Double (chữ D hoa) để có thể nhận giá trị null
    
    private Integer discount;
    
    private Integer rating; // Số sao (VD: 5)
    
    private Integer ratingCount; // Số lượt đánh giá (VD: 65)
    
    private String img; // Link ảnh
    
    private String category; // Danh mục
    
    private Integer stock; // Tồn kho

    // Các cờ phân loại để show ra trang chủ
    @Column(name = "is_flash_sale")
    private Boolean isFlashSale;

    @Column(name = "is_best_selling")
    private Boolean isBestSelling;

    @Column(name = "is_featured")
    private Boolean isFeatured;

    @Column(name = "is_new_arrival")
    private Boolean isNewArrival;

    public Long getId() {
        return id;
    }

    public void setId(Long id) {
        this.id = id;
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

    public Double getOldPrice() {
        return oldPrice;
    }

    public void setOldPrice(Double oldPrice) {
        this.oldPrice = oldPrice;
    }

    public Integer getDiscount() {
        return discount;
    }

    public void setDiscount(Integer discount) {
        this.discount = discount;
    }

    public Integer getRating() {
        return rating;
    }

    public void setRating(Integer rating) {
        this.rating = rating;
    }

    public Integer getRatingCount() {
        return ratingCount;
    }

    public void setRatingCount(Integer ratingCount) {
        this.ratingCount = ratingCount;
    }

    public String getImg() {
        return img;
    }

    public void setImg(String img) {
        this.img = img;
    }

    public String getCategory() {
        return category;
    }

    public void setCategory(String category) {
        this.category = category;
    }

    public Integer getStock() {
        return stock;
    }

    public void setStock(Integer stock) {
        this.stock = stock;
    }

    public Boolean getIsFlashSale() {
        return isFlashSale;
    }

    public void setIsFlashSale(Boolean isFlashSale) {
        this.isFlashSale = isFlashSale;
    }

    public Boolean getIsBestSelling() {
        return isBestSelling;
    }

    public void setIsBestSelling(Boolean isBestSelling) {
        this.isBestSelling = isBestSelling;
    }

    public Boolean getIsFeatured() {
        return isFeatured;
    }

    public void setIsFeatured(Boolean isFeatured) {
        this.isFeatured = isFeatured;
    }

    public Boolean getIsNewArrival() {
        return isNewArrival;
    }

    public void setIsNewArrival(Boolean isNewArrival) {
        this.isNewArrival = isNewArrival;
    }

    
}