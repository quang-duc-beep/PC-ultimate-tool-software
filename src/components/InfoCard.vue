<script setup>
// Nhận props từ component cha
defineProps({
  title: {
    type: String,
    required: true
  },
  icon: {
    type: [Object, Function], // Dành cho các icon từ lucide-vue-next
    required: true
  }
})
</script>

<template>
  <div class="border border-slate-200 rounded-2xl p-6 hover:shadow-lg hover:-translate-y-1 transition-all duration-300 bg-white group cursor-default flex flex-col">
    <!-- Phần Header của Card -->
    <div class="flex justify-between items-center mb-4 text-slate-700 group-hover:text-blue-600 transition-colors">
      <h2 class="text-xl font-bold">{{ title }}</h2>
      <component :is="icon" class="w-6 h-6" />
    </div>
    
    <!-- Phần Nội dung (được truyền vào từ cha qua thẻ slot) -->
    <div class="space-y-3 text-slate-600">
      <slot></slot>
    </div>
  </div>
</template>