<template lang="pug">
div.context-menu(
  v-if="contextMenu"
  @mousedown.stop.prevent
  @contextmenu.prevent
  ref="menuEl"
)
  a.option(
    v-for="item in contextMenu.menu"
    @click="()=>{item.fn(),contextMenu=undefined}"
  ) {{item.text}}
</template>

<script lang="ts" setup>
import { useStore } from "@/state";
import { createPopper } from "@popperjs/core";
import type { Instance, VirtualElement } from "@popperjs/core";
import { ref, toRef, watch } from "vue";

const store = useStore();
const contextMenu = toRef(store,"contextMenu");
const menuEl = ref<HTMLElement>();
let popper: Instance;

watch(menuEl, (el) => {
  if (!el) return popper.destroy();
  if (!contextMenu.value) return;

  const { x, y } = contextMenu.value;

  window.addEventListener("mousedown",()=>{    
    contextMenu.value = undefined,
    {once:true,passive:true}
  })
  
  const virtualElement = {
    getBoundingClientRect() {
      return {
        height: 0,
        width: 0,
        left: x,
        top: y,
      };
    },
  } as VirtualElement

  popper = createPopper(virtualElement, el, {
    placement: "right-start",
    modifiers: [
      {
        name: 'flip',
        options: {
          fallbackPlacements: ['right','left-start','left'],
        },
      },
    ]
  });
});
</script>

<style lang="scss" module>
.context-menu {
  z-index: 1000;
  background: rgba(rgb(20, 20, 20),.9);
  backdrop-filter: blur(2px);
  padding: .2em;
  border-radius: .4em;
  box-shadow: 0 0 .2em rgba(0,0,0,.5);

  & > .option {
    color: color.white();
    display: block;
    margin: .2em;
    padding: .2em .4em;
    cursor: pointer;
    border-radius: .2em;
    &:hover {
      background: color.side-2();
    }
  }
}
</style>
