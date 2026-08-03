<script setup>
import { MemoryStick } from 'lucide-vue-next'
import { useSysInfo } from '../../composables/useSysInfo'

const { sysInfo, bytesToGB } = useSysInfo()
</script>

<template>
  <div class="h-full flex flex-col cursor-default">
    <div class="flex justify-between items-center mb-4 text-slate-800">
      <h2 class="text-xl font-bold">Memory</h2>
      <MemoryStick class="w-7 h-7 text-blue-600" />
    </div>
    
    <p class="text-slate-600 font-semibold mb-2">Memory usage (%)</p>
    
    <div class="border border-slate-200 rounded-2xl h-48 flex items-center justify-center mb-8 shadow-inner bg-slate-50 hover:bg-slate-100 hover:border-blue-300 transition-colors duration-300 group">
      <span class="text-7xl font-extrabold tracking-tight group-hover:scale-105 transition-transform duration-300 text-slate-800">
        {{ sysInfo?.memory_usage_pre?.toFixed(1) ?? '--,-' }}%
      </span>
    </div>

    <div class="space-y-4 text-slate-700 text-lg">
      <p class="hover:translate-x-2 transition-transform duration-200"><span class="font-bold">Memory total:</span> {{ bytesToGB(sysInfo?.memory_total) }} GB</p>
      <p class="hover:translate-x-2 transition-transform duration-200"><span class="font-bold">Memory usage (byte):</span> {{ bytesToGB(sysInfo?.memory_usage) }} GB</p>
    </div>
  </div>
</template>