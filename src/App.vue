<script setup>
import Sidebar from './components/Sidebar.vue'
</script>

<template>
  <div class="flex h-screen bg-slate-50 text-slate-800 font-sans text-base lg:text-lg selection:bg-blue-200 cursor-default">
    <Sidebar />
    <main class="flex-1 overflow-y-auto relative bg-white m-3 rounded-2xl shadow-sm border border-slate-100 p-6 transition-all">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<style>
/* Hiệu ứng chuyển trang mượt mà */
.fade-enter-active, .fade-leave-active { transition: opacity 0.2s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>