<template lang="pug">
//- Title(@contextmenu.prevent)
Menu
div#main
  Aside(@contextmenu.prevent)
  main#view(@contextmenu.prevent="e => menu(e)"): RouterView
Popper
</template>

<script setup lang="ts">
// import Title from "@/layout/Title.vue";
import { useStore } from "@/state";
import Menu from "@/layout/Menu.vue";
import Aside from "@/layout/Aside.vue";
import Popper from "@/layout/Popper.vue";
// import { appWindow } from "@tauri-apps/api/window";

// let radius = false;
// function borderRadius(bool:boolean){
//   if (bool === radius) return;
//   radius = bool;
//   document.body.classList[bool ? "add" : "remove"]("radius")
// }

// onResize()
// appWindow.onResized(onResize)

// async function onResize() {
//   const isFullscreen = await appWindow.isFullscreen();
//   const isMaximized = await appWindow.isMaximized();

//   const isBorderRadius = isFullscreen || isMaximized;

//   console.log(isBorderRadius)

//   borderRadius(!isBorderRadius)
// }

const store = useStore()
function menu(e: MouseEvent) {
  store.setContentMenu(e, [
    {
      text: "重新整理",
      fn() { location.reload() },
    },
  ]);
  console.log(store.contextMenu);
}
</script>

<style lang="scss" module>
#main {
  display: flex;
  height: 100%;
  width: 100vw;
  overflow: hidden;
}

#view {
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden overlay;
}
</style>

<style src="@a/scss/page.scss" lang="scss"></style>