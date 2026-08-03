import { createApp } from 'vue'
import App from './App.vue'

// Import router mà chúng ta đã cấu hình
import { router } from './router' 

// Import file CSS của Tailwind (Tên file có thể khác tùy thuộc vào cách bạn setup Tailwind)
import './style.css' 

const app = createApp(App)

// BẮT BUỘC PHẢI CÓ DÒNG NÀY: Báo cho Vue biết để sử dụng Router
app.use(router) 

app.mount('#app')