// Hàm đếm ngược thời gian
function startFlashSaleCountdown(endDateString) {
    // Lấy thời gian kết thúc (đổi thành timestamp)
    const countDownDate = new Date(endDateString).getTime();

    // Cập nhật bộ đếm mỗi 1 giây (1000 milliseconds)
    const timerInterval = setInterval(function() {
        // Lấy thời gian hiện tại
        const now = new Date().getTime();

        // Tính khoảng thời gian còn lại
        const distance = countDownDate - now;

        // Nếu thời gian đã hết, dừng đếm và hiển thị thông báo
        if (distance < 0) {
            clearInterval(timerInterval);
            document.getElementById("countdown").innerHTML = "<div style='color: red; font-weight: bold;'>Flash Sale Đã Kết Thúc!</div>";
            return;
        }

        // Tính toán số Ngày, Giờ, Phút, Giây
        const days = Math.floor(distance / (1000 * 60 * 60 * 24));
        const hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
        const minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
        const seconds = Math.floor((distance % (1000 * 60)) / 1000);

        // Hiển thị ra HTML (Dùng padStart để luôn hiển thị 2 chữ số, VD: "03" thay vì "3")
        document.getElementById("flash-days").innerText = String(days).padStart(2, '0');
        document.getElementById("flash-hours").innerText = String(hours).padStart(2, '0');
        document.getElementById("flash-minutes").innerText = String(minutes).padStart(2, '0');
        document.getElementById("flash-seconds").innerText = String(seconds).padStart(2, '0');

    }, 1000);
}

// Khởi chạy khi tài liệu HTML đã load xong
document.addEventListener('DOMContentLoaded', function() {
    // 1. Khởi chạy Flash Sale (Ví dụ: 3 ngày nữa)
    let flashSaleDate = new Date();
    flashSaleDate.setDate(flashSaleDate.getDate() + 3);
    startFlashSaleCountdown(flashSaleDate);

    // 2. Khởi chạy Promo Banner (Ví dụ: 5 ngày nữa)
    let promoDate = new Date();
    promoDate.setDate(promoDate.getDate() + 5); // Cho thời gian dài hơn flash sale 1 chút
    startPromoCountdown(promoDate);
});

// --- LOGIC CUỘN DANH SÁCH FLASH SALE ---
function slideFlashSale(direction) {
    // Lấy ra thẻ div chứa danh sách sản phẩm
    const list = document.getElementById('flash-sale-list');

    // Khoảng cách cuộn mỗi lần nhấn (Bằng chiều rộng 1 card + khoảng cách gap)
    // Ở CSS card là 270px + gap 30px = 300px
    const scrollAmount = 300;

    if (list) {
        if (direction === 'left') {
            // Cuộn sang trái (trừ đi scrollAmount)
            list.scrollBy({
                left: -scrollAmount,
                behavior: 'smooth' // Cuộn mượt mà không bị giật cục
            });
        } else if (direction === 'right') {
            // Cuộn sang phải (cộng thêm scrollAmount)
            list.scrollBy({
                left: scrollAmount,
                behavior: 'smooth'
            });
        }
    }
}

// --- LOGIC CUỘN DANH SÁCH CATEGORY ---
function slideCategory(direction) {
    const list = document.getElementById('category-list');

    // Width của card là 170px + gap 30px = 200px
    const scrollAmount = 200;

    if (list) {
        if (direction === 'left') {
            list.scrollBy({ left: -scrollAmount, behavior: 'smooth' });
        } else if (direction === 'right') {
            list.scrollBy({ left: scrollAmount, behavior: 'smooth' });
        }
    }
}

// --- Hàm đếm ngược thời gian cho PROMO BANNER (Loa JBL) ---
function startPromoCountdown(endDateString) {
    const countDownDate = new Date(endDateString).getTime();

    const timerInterval = setInterval(function() {
        const now = new Date().getTime();
        const distance = countDownDate - now;

        if (distance < 0) {
            clearInterval(timerInterval);
            // Có thể ẩn bộ đếm hoặc hiện chữ Kết thúc tùy ý
            document.querySelector(".promo-banner__timer").innerHTML = "<h3 style='color: white;'>Khuyến mãi đã kết thúc!</h3>";
            return;
        }

        const days = Math.floor(distance / (1000 * 60 * 60 * 24));
        const hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
        const minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
        const seconds = Math.floor((distance % (1000 * 60)) / 1000);

        // Đổ dữ liệu vào các ID của Promo Banner
        document.getElementById("promo-days").innerText = String(days).padStart(2, '0');
        document.getElementById("promo-hours").innerText = String(hours).padStart(2, '0');
        document.getElementById("promo-minutes").innerText = String(minutes).padStart(2, '0');
        document.getElementById("promo-seconds").innerText = String(seconds).padStart(2, '0');

    }, 1000);
}