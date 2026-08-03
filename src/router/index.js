import { createRouter, createWebHashHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Performance from '../views/Performance.vue'
import Cpu from '../views/performance/Cpu.vue'
import Memory from '../views/performance/Memory.vue'
import Disk from '../views/performance/Disk.vue'
import DiskCleanup from '../views/DiskCleanup.vue'
import Information from '../views/Information.vue'

const routes = [
  { path: '/', component: Home },
  { 
    path: '/performance', 
    component: Performance,
    redirect: '/performance/cpu',
    children: [
      { path: 'cpu', component: Cpu },
      { path: 'memory', component: Memory },
      { path: 'disk', component: Disk }
    ]
  },
  { path: '/cleanup', component: DiskCleanup },
  { path: '/information', component: Information }
]

export const router = createRouter({
  history: createWebHashHistory(),
  routes
})