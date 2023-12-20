import { RouteRecordRaw, createRouter, createWebHistory } from 'vue-router';

import MainLayout from '@/layout/main.vue';
import Index from '@/views/index/index.vue';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'mainLayout',
    component: MainLayout,
    redirect: '/',
    children: [
      {
        path: '/',
        name: 'index',
        component: Index,
        redirect: 'mv',
        children: [
          {
            path: 'localMusic',
            name: 'localMusic',
            component: () => import('@/views/local/index.vue')
          },
          {
            path: 'iLike',
            name: 'iLike',
            component: () => import('@/views/like/index.vue')
          },
          {
            path: 'searchResult',
            name: 'searchResult',
            component: () => import('@/views/search/index.vue')
          },
          {
            path: 'mv',
            name: 'mv',
            component: () => import('@/views/mv/index.vue')
          }
        ]
      }
    ]
  }
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;
