<template lang="pug">
div#title-bar(@mousedown="appWindow.startDragging")
  h1.title SteamTools
  div(@mousedown.stop @contextmenu.prevent).control
    button.min(@click="minimize" tabindex="-1")
      svg(viewBox="0 0 16 16"): path(d="M4 8a.5.5 0 0 1 .5-.5h7a.5.5 0 0 1 0 1h-7A.5.5 0 0 1 4 8z")
    button.max(@click="maximize" tabindex="-1")
      svg(viewBox="0 0 16 16"): rect(x="4" y="4" width="8" height="8" fill-rule="evenodd")
    button.close(@click="close" tabindex="-1")
      svg(viewBox="0 0 16 16"): path(d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z")
</template>

<script lang="ts" setup>
import { appWindow } from "@tauri-apps/api/window";

function close(){ appWindow.close() }
function minimize(){ appWindow.minimize() }
function maximize(){ appWindow.toggleMaximize() }
</script>

<style lang="scss" module>
#title-bar {
  width: 100%;
  height: auto;
  display: flex;
  background: color.side-1();
  justify-content: space-between;
  padding-left: 4px;

  .title {
    user-select: none;
    font-size: 1em;
    margin: .2em;
    color: color.bg();
    font-weight: 800;
  }

  .control {
    display: flex;
    & > button {
      display: block;
      border: none;
      background: transparent;

      & > svg {
        vertical-align: middle;
        width: 1.8em;
        height: 1.4em;
        & > path {
          fill: color.black();
        }
        & > rect {
          stroke:color.black();
          fill: transparent;
        }
      }
      &:hover {
        background: color.black(.8);
        & > svg {
          & > path {
            fill: color.white();
          }
          & > rect {
            stroke:color.white();
            fill: transparent;
          }
        }
      }
      &.close:hover { background: #dc4b4b; }
    }
  }
}

</style>
