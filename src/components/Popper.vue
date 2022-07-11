<template lang="pug">
div(
  ref="item"
  @mouseenter="hover = true"
  @mouseleave="hover = false"
  v-bind="$attrs"
)
  slot
  Teleport(:to="teleport" v-if="teleport")
    .tooltip(:class="{hover}" ref="tooltip")
      .tooltip-inner(v-if="tooltipSlot"): slot(name="tooltip")
      .tooltip-inner(v-else) {{props.tooltip}}
</template>

<script lang="ts" setup>
import { ref, toRef, useAttrs, useSlots, watch } from "vue";
import { createPopper } from "@popperjs/core";
import type { Placement } from "@popperjs/core";
import { useStore } from "@/state";

const hover = ref(false);
const teleport = toRef(useStore(),"tooltipEl");

const { tooltip:tooltipSlot } = useSlots()

const props = withDefaults(
  defineProps<{
    placement?: Placement;
    tooltip?: string;
  }>(),
  {
    placement: "auto",
    tooltip: "undefined",
  }
);

const tooltip = ref<HTMLElement>();
const item = ref<HTMLElement>();

watch([tooltip,item],() => {
  if (!item.value || !tooltip.value) return;
  createPopper(item.value, tooltip.value, {
    placement: props.placement,
    modifiers: [
      {
        name: "offset",
        options: {
          offset: [0, 8],
        },
      }
    ]
  });
});
</script>

<style lang="scss" module>
.tooltip {
  pointer-events: none;
  user-select: none;
  word-break: keep-all;
  padding: .2em .6em .2em;
  position: fixed;
  margin: 0;
  border-radius: .4em;
  background: color.black();
  box-shadow: 0 0 .2em 2px color.bg();

  transition: opacity .1s;
  opacity: 0;

  &::before {
    content: "";
    width: .1em;
    height: 100%;
    background: color.focus();
    box-shadow: 0 0 .2em color.focus(.5);
  }
}

.tooltip-inner {
  color: color.white();
}

.tooltip.hover {
  animation: tooltip .3s backwards;
  visibility: visible;
  opacity: 1;
}

@keyframes tooltip {
  from {
    visibility: visible;
    opacity: 0;
  }
  to {
    visibility: visible;
    opacity: 1;
  }
}
</style>