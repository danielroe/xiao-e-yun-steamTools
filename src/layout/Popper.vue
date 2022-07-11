<template lang="pug">
.tooltip-group(ref="tooltip")
</template>

<script lang="ts" setup>
import { useStore } from '@/state';
import { onMounted, ref } from 'vue';

const store = useStore()
const tooltip = ref<HTMLDivElement>()

onMounted(() => {
  store.tooltipEl = tooltip.value;
})
</script>

<style lang="scss" module>
.tooltip-group {
  overflow: visible;
  position: fixed;
  z-index: 100;
}
</style>