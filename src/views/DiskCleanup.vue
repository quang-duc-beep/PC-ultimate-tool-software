<script setup>
import { HardDrive } from 'lucide-vue-next'
</script>

<template>
  <div class="h-full flex flex-col">
    <h1 class="text-2xl font-bold flex items-center gap-3 mb-6 text-slate-800">
      <HardDrive class="w-8 h-8 text-blue-600" /> Disk cleanup
    </h1>
    
    <div class="flex-1 flex flex-col items-center justify-center text-center group cursor-default">
      <!-- Hiệu ứng nảy lặp lại nhẹ -->
      <h2 class="text-6xl font-extrabold text-slate-800 mb-4 group-hover:text-blue-600 transition-colors duration-500">Comming soon!</h2>
      <p class="text-2xl text-slate-500 font-medium group-hover:scale-105 transition-transform duration-300">Disk cleanup will comming next update!</p>
    </div>
  </div>
</template>