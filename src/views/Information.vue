<script setup>
import { Info } from 'lucide-vue-next'
import { useSysInfo } from '../composables/useSysInfo'

const { sysInfo, wmiDisk } = useSysInfo()
</script>

<template>
  <div class="h-full flex flex-col cursor-default">
    <h1 class="text-2xl font-bold flex items-center gap-3 mb-8 text-slate-800">
      <Info class="w-8 h-8 text-blue-600" /> PC information
    </h1>

    <div class="space-y-5 text-xl text-slate-700 max-w-2xl bg-slate-50 p-8 rounded-2xl border border-slate-100 shadow-sm hover:shadow-md transition-shadow">
      <p class="flex items-center group"><span class="font-bold w-48 group-hover:text-blue-600 transition-colors">Name OS:</span> {{ sysInfo?.name_os ?? '....' }}</p>
      <p class="flex items-center group"><span class="font-bold w-48 group-hover:text-blue-600 transition-colors">Name PC:</span> {{ sysInfo?.name_pc ?? '....' }}</p>
      <p class="flex items-center group"><span class="font-bold w-48 group-hover:text-blue-600 transition-colors">Sys Manufacturer:</span> N/A</p>
      <p class="flex items-center group"><span class="font-bold w-48 group-hover:text-blue-600 transition-colors">System Type:</span> {{ sysInfo?.sys_type ?? '...' }}</p>
      <p class="flex items-center group"><span class="font-bold w-48 group-hover:text-blue-600 transition-colors">Disk name:</span> {{ wmiDisk[0]?.model ?? '...' }}</p>
      <p class="flex items-center group"><span class="font-bold w-48 group-hover:text-blue-600 transition-colors">Disk space:</span> {{ wmiDisk[0]?.size_gb ?? '---' }}GB</p>
    </div>

    <div class="mt-auto pb-10 flex justify-center text-slate-400 font-medium text-lg hover:text-slate-600 transition-colors duration-300">
      That's all there is to it.
    </div>
  </div>
</template>