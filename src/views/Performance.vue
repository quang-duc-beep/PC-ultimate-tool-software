<script setup>
import { Activity, Cpu, MemoryStick, HardDrive } from 'lucide-vue-next'
import { useRoute } from 'vue-router'

const route = useRoute()
const subMenus = [
  { path: '/performance/cpu', icon: Cpu },
  { path: '/performance/memory', icon: MemoryStick },
  { path: '/performance/disk', icon: HardDrive },
]
</script>

<template>
  <div class="h-full flex flex-col relative pb-20">
    <h1 class="text-2xl font-bold flex items-center gap-3 mb-6 text-slate-800">
      <Activity class="w-8 h-8 text-blue-600" /> Performance
    </h1>

    <div class="flex-1">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </div>

    <!-- Floating Tab Bar (Menu dưới cùng) -->
    <div class="absolute bottom-4 left-1/2 -translate-x-1/2 bg-slate-200/90 backdrop-blur-md rounded-2xl p-2 flex gap-2 shadow-lg border border-slate-300">
      <router-link 
        v-for="menu in subMenus" 
        :key="menu.path" 
        :to="menu.path"
        class="relative p-4 rounded-xl transition-all duration-300 hover:bg-slate-300 group"
      >
        <component 
          :is="menu.icon" 
          class="w-8 h-8 transition-transform duration-300 group-hover:scale-110 group-hover:-translate-y-1" 
          :class="route.path === menu.path ? 'text-blue-600' : 'text-slate-600'" 
        />
        <!-- Dấu gạch dưới báo hiệu active -->
        <span 
          v-if="route.path === menu.path" 
          class="absolute bottom-2 left-1/2 -translate-x-1/2 w-6 h-1.5 bg-blue-500 rounded-full"
        ></span>
      </router-link>
    </div>
  </div>
</template>