<script setup>
import { Home, Cpu, MemoryStick, HardDrive, Info } from 'lucide-vue-next'
import { useSysInfo } from '../composables/useSysInfo'
import InfoCard from '../components/InfoCard.vue' // Import component dùng chung

const { sysInfo, formatUptime, bytesToGB } = useSysInfo()
</script>

<template>
  <div class="h-full flex flex-col">
    <h1 class="text-2xl font-bold flex items-center gap-3 mb-6 text-slate-800">
      <Home class="w-8 h-8 text-blue-600" /> Home
    </h1>

    <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 flex-1 content-start">
      
      <!-- Card CPU -->
      <InfoCard title="Cpu" :icon="Cpu">
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Utilization:</span> {{ sysInfo?.cpu_usage?.toFixed(1) ?? '--,-' }}%</p>
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Up time:</span> {{ formatUptime(sysInfo?.cpu_uptime) }}</p>
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Base speed:</span> {{ sysInfo?.cpu_freq?.toFixed(2) ?? '--,-' }}GHz</p>
      </InfoCard>

      <!-- Card Memory -->
      <InfoCard title="Memory" :icon="MemoryStick">
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Memory usage(GB):</span> {{ bytesToGB(sysInfo?.memory_usage) }} GB</p>
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Memory usage(%):</span> {{ sysInfo?.memory_usage_pre?.toFixed(1) ?? '--,-' }}%</p>
      </InfoCard>

      <!-- Card Disk -->
      <InfoCard title="Disk" :icon="HardDrive">
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Name disk:</span> {{ sysInfo?.disk_name ?? '....' }}</p>
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Free space:</span> {{ sysInfo?.free_space?.toFixed(1) ?? '---' }} GB</p>
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Space used:</span> {{ sysInfo?.space_used?.toFixed(1) ?? '---' }} GB</p>
      </InfoCard>

      <!-- Card Information -->
      <InfoCard title="Information" :icon="Info">
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Name OS:</span> {{ sysInfo?.name_os ?? '....' }}</p>
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Name PC:</span> {{ sysInfo?.name_pc ?? '....' }}</p>
        <p class="hover:translate-x-1 transition-transform"><span class="font-semibold">Sys Type:</span> {{ sysInfo?.sys_type ?? 'N/A' }}</p>
      </InfoCard>

    </div>
  </div>
</template>