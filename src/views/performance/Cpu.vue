<script setup>
import { Cpu } from 'lucide-vue-next'
import { useSysInfo } from '../../composables/useSysInfo'

const { sysInfo, formatUptime } = useSysInfo()
</script>

<template>
  <div class="h-full flex flex-col cursor-default">
    <div class="flex justify-between items-center mb-4 text-slate-800">
      <h2 class="text-xl font-bold">Cpu</h2>
      <Cpu class="w-7 h-7 text-blue-600" />
    </div>
    
    <p class="text-slate-600 font-semibold mb-2">Utilization</p>
    
    <!-- Box hiển thị lớn -->
    <div class="border border-slate-200 rounded-2xl h-48 flex items-center justify-center mb-8 shadow-inner bg-slate-50 hover:bg-slate-100 hover:border-blue-300 transition-colors duration-300 group">
      <span class="text-7xl font-extrabold tracking-tight group-hover:scale-105 transition-transform duration-300 text-slate-800">
        {{ sysInfo?.cpu_usage?.toFixed(1) ?? '--,-' }}%
      </span>
    </div>

    <!-- Chia cột thông tin chi tiết -->
    <div class="grid grid-cols-2 gap-x-12 gap-y-6 text-slate-700 text-lg">
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Utilization:</span> {{ sysInfo?.cpu_usage?.toFixed(1) ?? '--,-' }}%</p>
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Logical Core:</span> {{ sysInfo?.cpu_core ?? '---' }}</p>
      
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Base speed:</span> {{ sysInfo?.cpu_freq?.toFixed(2) ?? '--,-' }}GHz</p>
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Logical processor:</span> {{ sysInfo?.cpu_processor ?? '---' }}</p>
      
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Name processor:</span> {{ sysInfo?.sys_type ?? '...' }}</p>
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Up time:</span> {{ formatUptime(sysInfo?.cpu_uptime) }}</p>
    </div>
  </div>
</template>