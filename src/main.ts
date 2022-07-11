import { createApp } from "vue";
import router from "@/router";
import App from "@/layout/App.vue";
import Popper from "@/components/Popper.vue";
import { createPinia } from "pinia";

createApp(App)
  .component("Popper", Popper)
  .use(router)
  .use(createPinia())
  .mount("#app");

declare module 'vue' {
  export interface GlobalComponents {
    Popper: typeof Popper,
  }
}