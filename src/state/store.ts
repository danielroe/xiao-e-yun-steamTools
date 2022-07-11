export const state: State = {
  contextMenu: undefined,
  tooltipEl: undefined,
}

interface State {
  contextMenu?:{
    x: number,
    y: number,
    menu: {
      text: string,
      fn: ()=>void,
    }[],
  },
  tooltipEl?: HTMLElement,
}