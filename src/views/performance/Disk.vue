<script setup>
import { HardDrive } from 'lucide-vue-next'
import { useSysInfo } from '../../composables/useSysInfo'

const { sysInfo, wmiDisk } = useSysInfo()
</script>

<template>
  <div class="h-full flex flex-col cursor-default">
    <div class="flex justify-between items-center mb-4 text-slate-800">
      <h2 class="text-xl font-bold">Disk</h2>
      <HardDrive class="w-7 h-7 text-blue-600" />
    </div>
    
    <p class="text-slate-600 font-semibold mb-2">Space used</p>
    
    <div class="border border-slate-200 rounded-2xl h-48 flex items-center justify-center mb-8 shadow-inner bg-slate-50 hover:bg-slate-100 hover:border-blue-300 transition-colors duration-300 group">
      <span class="text-7xl font-extrabold tracking-tight group-hover:scale-105 transition-transform duration-300 text-slate-800">
        {{ sysInfo?.space_used?.toFixed(1) ?? '---' }}GB
      </span>
    </div>

    <div class="grid grid-cols-2 gap-x-12 gap-y-6 text-slate-700 text-lg">
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Model disk name:</span> {{ wmiDisk[0]?.model ?? 'N/A' }}</p>
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Total disk:</span> {{ wmiDisk[0]?.size_gb ?? '---' }}GB</p>

      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Space used:</span> {{ sysInfo?.space_used?.toFixed(1) ?? '---' }}GB</p>
      <p class="hover:text-blue-600 transition-colors"><span class="font-bold">Model disk:</span> {{sysInfo?.disk_kind ?? '...' }}</p>
    </div>
  </div>
</template>