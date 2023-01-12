# Ứng dụng chuyển dữ liệu ZaloPC sang ổ đĩa khác
Mặc định Zalo cài vào ổ C và nằm trong thư mục tạm của máy. Khi sử dụng ZaloPC, cho dù người dùng có tải về tài liệu hay hình ảnh thì tất cả tập tin đều được lưu vào thư mục %localappdata% của người dùng. Về lâu dài chính những tập tin tạm này sẽ khiến ổ C đầy lên gây lỗi chiếm rất nhiều dung lượng.
Có hai hướng giải quyết cho vấn đề này:
- Hướng 1: Xoá ZaloPC và các thư mục tạm đi kèm, chuyển sang dùng Zalo nền web.
- Hướng 2: Chuyển dữ liệu tạm trong ZaloPC sang ổ cứng khác có dung lượng lớn hơn và tự xoá định kỳ khi đầy.

Chương trình này được viết ra để thực hiện các thao tác phức tạp của Hướng 2 dành cho người không rành về câu lệnh và hệ thống thư mục.
Hướng dẫn sử dụng:

- Bước 1: Tải MoveZaloData.zip về rồi giải nén để có MoveZaloData.exe.
- Bước 2: Thoát hoàn toàn Zalo, nhớ kiểm tra biểu tượng Zalo nhỏ ở góc phải xem đã mất chưa.
- Bước 3: Chạy MoveZaloData.exe bằng quyền Administration.
- Bước 4: Nhập ổ cứng bạn muốn chuyển ZaloPC sang. Ví dụ ổ D có dung lượng trống nhiều, bạn chỉ cần nhấn d rồi Enter.
- Bước 5: Bấm Yes để tiếp tục, kiểm tra tên đường dẫn ở cửa sổ xem có đúng ổ đĩa bạn chọn chưa.
- Bước 6: Đợi đến khi có thông báo hoàn tất. Bấm OK rồi mở lại Zalo.

Lưu ý: Sau khi chạy xong chương trình, bạn vào ổ cứng kiểm tra xem đã có thư mục ZaloPC được chuyển qua chưa. Nếu có xuất hiện thư mục ZaloPC thì việc chuyển thành công. Không chạy lại chương trình này nữa (nếu chạy thêm lần nữa thì dử liệu sẽ bị xoá mất).

Chúc các bạn thành công.