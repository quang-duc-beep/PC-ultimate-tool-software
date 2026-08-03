<script setup>
import { Home, Activity, HardDrive, Info, LifeBuoy, Settings } from 'lucide-vue-next'
import { useRoute } from 'vue-router'

const route = useRoute()

const menuItems = [
  { name: 'Home', path: '/', icon: Home },
  { name: 'Performance', path: '/performance', icon: Activity },
  { name: 'Disk cleanup', path: '/cleanup', icon: HardDrive },
  { name: 'Information', path: '/information', icon: Info },
]

const bottomItems = [
  { name: 'Support', icon: LifeBuoy },
  { name: 'Setting', icon: Settings },
]

// Hàm kiểm tra path hiện tại (Hỗ trợ active cho thẻ cha Performance)
const isActive = (path) => {
  if (path === '/') return route.path === '/'
  return route.path.startsWith(path)
}
</script>

<template>
  <aside class="w-53 flex-shrink-0 flex flex-col justify-between border-r border-slate-200 bg-slate-50 p-4 transition-all">
    <div>
      <!-- Logo dạng img, có hiệu ứng nảy nhẹ khi hover -->
      <div class="mb-8 px-2 cursor-pointer group">
        <img src="../assets/Logo-AVFinfo.png" alt="App Logo" class="w-16 h-16 rounded-2xl shadow-md border border-slate-200 group-hover:scale-105 group-hover:shadow-lg transition-all duration-300" />
      </div>

      <nav class="space-y-2">
        <router-link 
          v-for="item in menuItems" 
          :key="item.path" 
          :to="item.path"
          class="flex items-center gap-4 px-4 py-3 rounded-xl font-medium transition-all duration-200 group relative overflow-hidden"
          :class="isActive(item.path) ? 'bg-slate-200 text-blue-700 font-semibold' : 'text-slate-600 hover:bg-slate-100 hover:text-blue-600 hover:translate-x-1'"
        >
          <component :is="item.icon" class="w-6 h-6 transition-transform group-hover:scale-110" />
          <span class="text-lg">{{ item.name }}</span>
        </router-link>
      </nav>
    </div>

    <div class="space-y-2 border-t border-slate-200 pt-4">
      <div 
        v-for="item in bottomItems" 
        :key="item.name"
        class="flex items-center gap-4 px-4 py-3 rounded-xl font-medium text-slate-600 hover:bg-slate-100 hover:text-blue-600 hover:translate-x-1 transition-all duration-200 cursor-pointer group"
      >
        <component :is="item.icon" class="w-6 h-6 group-hover:scale-110 transition-transform" />
        <span class="text-lg">{{ item.name }}</span>
      </div>
    </div>
  </aside>
</template>