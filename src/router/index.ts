import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'works',
      component: () => import('../views/WorksListPage.vue'),
    },
    {
      path: '/works/:id',
      name: 'work',
      component: () => import('../views/WorkPage.vue'),
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/SettingsPage.vue'),
    },
  ],
})

export default router
