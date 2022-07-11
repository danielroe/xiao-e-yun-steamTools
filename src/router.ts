import { createRouter, createWebHistory } from 'vue-router';
import index from "@p/index.vue";
import login from "@p/login.vue";
import settings from "@p/settings.vue";
import twoFa from "@p/2fa.vue";

const routes = [
  { path: '/', component: index },
  { path: '/2fa', component: twoFa },
  { path: '/login', component: login },
  { path: '/settings', component: settings },
]

export default createRouter({
  history: createWebHistory(),
  routes,
})