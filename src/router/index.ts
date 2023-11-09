import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';

import MainLayout from '@/layout/main.vue';
import Index from '@/views/index/index.vue';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'mainLayout',
    component: MainLayout,
    redirect: '/',
    children: [{ path: '/', name: 'index', component: Index }],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
