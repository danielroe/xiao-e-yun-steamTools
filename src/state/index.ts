import { defineStore } from 'pinia'
import { reactive } from 'vue'
import { state } from './store'

export const useStore = defineStore("store",{
  state: ()=> state,
  actions: {
    async setContentMenu(
      event:MouseEvent,
      list:{text:string,fn: ()=>void}[]
    ){
      
      this.contextMenu = {
        x: event.clientX,
        y: event.clientY,
        menu: list,
      }
          
    }
  },
})