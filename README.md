<img width="150" height="150" alt="Logo-AVFinfo" src="https://github.com/user-attachments/assets/efc74d4a-a5af-44d4-86c1-5f14f17fcd21" />

# 🖥️PC Ultimate Tool Software

## Giới thiệu
**PC Ultimate Tool Software** là một công cụ tiện ích dành cho máy tính cá nhân hệ điều hành Windows, cung cấp cho người dùng một giao diện trực quan và hiện đại để theo dõi, quản lý và kiểm tra tình trạng hệ thống. Với thiết kế tối giản, ứng dụng giúp bạn nắm bắt nhanh chóng các thông số phần cứng quan trọng nhất của máy tính trong thời gian thực.
- **Founder**: [ProjectForge - Khuất Quang Đức](https://github.com/quang-duc-beep)
- **Số lượng thành viên xây dựng**: 01

## Công cụ tạo nên sản phẩm
- **frontend**: Vue3
- **backend**: rust + tauri

<p align="center">
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/Rust-000000?style=plastic&logo=rust&logoColor=white" alt="Rust">
  </a>
  <a href="https://tauri.app/">
    <img src="https://img.shields.io/badge/Tauri-24C8DB?style=plastic&logo=tauri&logoColor=white" alt="Tauri">
  </a>
  <a href="https://vuejs.org/">
    <img src="https://img.shields.io/badge/Vue.js-4FC08D?style=plastic&logo=vuedotjs&logoColor=white" alt="Vue.js">
  </a>
</p>

## Mô tả phần mềm và tính năng đang có
Phần mềm được thiết kế với menu điều hướng bên trái, bao gồm các nhóm tính năng chính hiện tại:
- **Home (Trang chủ):** Cung cấp cái nhìn tổng quan (Overview) về trạng thái hiện tại của CPU, Bộ nhớ (RAM) và Ổ cứng (Disk).
- **Performance (Hiệu năng):** Theo dõi chuyên sâu theo thời gian thực về:
  - **CPU:** Mức độ sử dụng (Utilization), thời gian hoạt động (Up time), tốc độ cơ bản (Base speed), số lõi logic (Logical Core/Processor).
  - **Memory (RAM):** Phần trăm sử dụng, tổng dung lượng hiện có, và dung lượng đã dùng (tính bằng GB).
  - **Disk (Ổ cứng):** Dung lượng trống, dung lượng đã sử dụng và tên ổ đĩa.
- **Information (Thông tin hệ thống):** Hiển thị các thông tin chi tiết về thiết bị như Tên hệ điều hành (Name OS), Tên máy tính (Name PC), Nhà sản xuất (Sys Manufacturer), Loại hệ thống (System Type).
- **Các mục chờ phát triển:** Disk cleanup (Dọn dẹp ổ đĩa), Support (Hỗ trợ), và Setting (Cài đặt).

## Ảnh

**Giao diện Tổng quan (Home)**  
<img width="1257" height="925" alt="image" src="https://github.com/user-attachments/assets/d67de0c0-849c-475a-978e-c646f29c3670" />
<img width="1917" height="1078" alt="image" src="https://github.com/user-attachments/assets/d82b33e0-8c69-4f78-8bca-588ba96f5a1b" />


**Giao diện Hiệu năng (Performance)**  
*CPU:*  
<img width="1253" height="913" alt="image" src="https://github.com/user-attachments/assets/41e9af6f-8e15-4307-a1df-cca2e8f04b84" />


*Memory:*  
<img width="1253" height="922" alt="image" src="https://github.com/user-attachments/assets/c1849703-e0ec-4700-aca9-63956004cbc0" />


*Disk:*  
<img width="1261" height="925" alt="image" src="https://github.com/user-attachments/assets/b1e43c08-ce77-499f-8d15-aa0469a96ca7" />


**Giao diện Thông tin hệ thống (Information)**  
<img width="1262" height="918" alt="image" src="https://github.com/user-attachments/assets/5b93dcf9-1a40-4f0f-ad43-3eaafe358944" />


## Quá trình xây dựng

**Bắt đầu phát triển với một ý tưởng**: Khi tôi đang chán nản ở nhà với một chiếc máy tính, tôi bắt đầu có một suy nghĩ để thoả mãn đam mê của mình, chính là xây dựng một phần mềm với công cụ hiện tại, vừa nhẹ lại còn ăn ít RAM, thế là tôi nghĩ đến ý tưởng "À hay là làm một phần mềm giúp mọi người có thể theo dõi thông số máy tính mà mình đang sử dụng" Thế là tôi đã bắt đầu ý tưởng đó thành sản phẩm thực tế

**Xác định điểm yếu và khắc phục bằng Ai đúng cách**: Khi tôi học công cụ Rust-tauri-vue và bắt đầu làm, tôi nhận ra rằng mình chả có thẩm mỹ về giao diện cả, vì lúc đó tôi đang thiết kế giao diện bằng code, nhưng nó khó với tôi, lúc thì phải chỉnh mấy đối tượng, font phù hợp, màu, bảng, kích thước,... Những công việc đó làm tôi điên lên vì thấy nó chả ra hồn gì. Thế là lại chán, nhưng lại chán thì lại có cái cứu tôi, đó là Ai, và thế là tôi đã sử dụng Ai làm frontend giúp tôi, từ thiết kế giao diện cho đến gọi sự kiện từ rust. Nhưng trước khi làm, tôi phải code rust trước. Sau hơn chục cái lỗi nghiêm khắc của rust và bug lặp lại, tôi đã thành công. Tiếp theo tôi thiết kế giao diện cơ bản trên figma, coi nó là "mẫu". Sau đó tôi "ép" Ai phải làm đúng yêu cầu từ code (cấm lệnh hardcode và code phức tạp), bắt nó phải code để bảo trì dễ, thêm comment,... cho đến gọi dữ liệu từ rust như một sếp khó tính. Tôi sử dụng Gemini và không ngờ, nó làm đúng tôi muốn

**Nghiên cứu và đọc code, sửa lỗi frontend của Ai**: trước khi tôi copy code của Ai vào, tôi phải xem nó có gì, nó viết code như nào, đúng yêu cầu không, và tôi copy kiểm thử xem nó có lỗi gì, cách sửa như nào, có chỗ nào sai không,... Để tôi bắt nó làm lại

**Quyết tâm no vibes code**: Đúng là tôi copy paste code từ Ai, nhưng đó là frontend, phần còn lại là do tôi tự code, tự mày mò (như lên trên mạng, nhờ Ai liệt kê các lệnh trong thư viện đó,...). Còn lỗi, tôi chỉ dùng Ai khi gặp lỗi khó hiểu hoặc quá nghiêm trọng. Tôi phải sử dụng Ai đúng cách để sản phẩm của tôi phải có xây dựng từ bàn tay của mình, chứ không phải nhờ Ai làm hết và nhìn sản phẩm để "kích thích dopamine trong não", sản phẩm code từ Ai chỉ là nửa của mục tiêu giải quyết điểm yếu thôi... 

**Thành công**: sau 3 ngày mày mò và nghiên cứu, cuối cùng tôi với Ai thành công xây dựng được một sản phẩm mong muốn, đó là điều tôi vui nhất sau bao năm, bao tháng suy nghĩ...

## Lỗi cần lưu ý
- Sai số dữ liệu từ disk, cpu,...
- Tên không hiển thị vì lỗi từ giữa UTF-8 của rust và UTF-16 của windows...

## Tính năng hiện tại và cập nhật sắp tới
**Tính năng hiện tại:**
* Theo dõi và giám sát tài nguyên hệ thống (CPU, RAM, Disk) theo thời gian thực.
* Truy xuất và hiển thị thông tin chi tiết về phần cứng, hệ điều hành.
* Giao diện UI/UX trực quan, hiện đại, chuyển đổi mượt mà giữa các chức năng.

**Cập nhật sắp tới:**
* Sửa lại các tính năng và sai số, lỗi...
* cập nhật các tính năng còn lại...
* Thêm một tính năng mới sắp tới...

## Phiên bản
- **Phiên bản hiện tại**: 0.1.0
- **ID Software**: 0.1.0-AVFSofw@re-PUTS
(c)

## Liên hệ ý kiến lắng nghe
- **Email**: ducps125@gmail.com
- **Facebook**: [click here](https://www.facebook.com/Qducbug/?locale=vi_VN)
- **Zalo**: [click here](https://zalo.me/0363829426)

- Cảm ơn bạn đã trải nghiệm phần mềm, đừng quên ủng hộ đánh giá ⭐ và 👁️ nhé!

<img width="450" height="339" alt="AVF-AlphaVForge" src="https://github.com/user-attachments/assets/475e6476-ff44-4627-8ef2-822413952818" />
