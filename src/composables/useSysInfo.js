import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core' // Dùng cho Tauri v2

export function useSysInfo() {
  const sysInfo = ref(null)
  let interval = null

  const fetchInfo = async () => {
    try {
      sysInfo.value = await invoke('sys_info')
    } catch (error) {
      console.error('Lỗi khi gọi Rust:', error)
    }
  }

  onMounted(() => {
    fetchInfo()
    interval = setInterval(fetchInfo, 1000) // Cập nhật mỗi 1 giây
  })

  onUnmounted(() => {
    if (interval) clearInterval(interval)
  })

  // Hàm chuyển đổi giây thành HH:MM:SS
  const formatUptime = (seconds) => {
    if (seconds == null) return '--:--:--'
    const h = Math.floor(seconds / 3600)
    const m = Math.floor((seconds % 3600) / 60)
    const s = seconds % 60
    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`
  }

  // Chuyển đổi Bytes sang GB
  const bytesToGB = (bytes) => {
    if (bytes == null) return '--,-'
    return (bytes / (1024 ** 3)).toFixed(1)
  }

  return { sysInfo, formatUptime, bytesToGB }
}