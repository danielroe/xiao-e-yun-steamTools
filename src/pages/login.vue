<template lang="pug">
div.list(
  :class="{loading}"
  @contextmenu.stop.prevent="e=>listMenu(e)"
)
  Popper.card(
    v-for="(user, index) in users"
    :key="user.SteamID"
    :style="{'--delay':index}"
    :class="{current:current === user.SteamID}"
    @click="replace(user.SteamID)"
    @contextmenu.stop.prevent="e=>userMenu(e,user.SteamID)"
    placement="bottom-start"
    :tooltip="user.AccountName"
  )
    h2.title(
      v-text="user.PersonaName"
    )
    img.avatar(v-if="user.Avatar" :src="convertFileSrc(user.Avatar)")
    svg.none-avatar(v-else viewBox="0 0 16 16"): path(d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0zm4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4zm-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10c-2.29 0-3.516.68-4.168 1.332-.678.678-.83 1.418-.832 1.664h10z")
</template>

<script lang="ts" setup>
import { useStore } from "@/state";
import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
import { onMounted, reactive, ref } from "vue";
const store = useStore();

const loading = ref(false);

const current = ref<string>();
const users = ref<
  {
    Avatar: string | null;
    SteamID: string;
    AccountName: string;
    PersonaName: string;
    Timestamp: number;
    MostRecent: number;
    AllowAutoLogin: number;
    WantsOfflineMode: number;
    RememberPassword: number;
    SkipOfflineModeWarning: number;
  }[]
>();
onMounted(async () => {
  current.value = await invoke("steamAccountCurrent");
  users.value = reactive(JSON.parse(await invoke("steamUsers")));  
});

async function replace(id: string) {
  if (id !== "" && current.value === id) return;
  loading.value = true;
  await invoke("steamAccountCurrent", { steamid: id });
  loading.value = false;
  current.value = id;
}

function userMenu(e: MouseEvent, steamid: string) {
  const others =
    steamid === current.value
      ? [
          {
            text: "登出Steam",
            async fn() {
              await replace("");
            },
          },
        ]
      : [];

  const mixin = [
    ...others,
    {
      text: "刪除帳號",
      async fn() {
        await invoke("steamAccountDelete", { steamid });
        users.value = reactive(JSON.parse(await invoke("steamUsers")));
      },
    },
  ]

  store.setContentMenu(e,mixin);
}

function listMenu(e: MouseEvent) {
  const list = [
      {
        text: "重啟Steam",
        fn() { invoke("steamRestart") },
      },
      {
        text: "刷新列表",
        async fn() {
          current.value = await invoke("steamAccountCurrent");
          users.value = reactive(JSON.parse(await invoke("steamUsers")));
        },
      },
    ]

  store.setContentMenu(e,list);
}
</script>

<style lang="scss" module>
.list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(15em, 1fr));
  grid-auto-rows: max-content;
  transition: opacity 1s;
  min-height: 100%;
  padding: 2em;
  gap: 2em;

  @include pe {
    font-size: .8em;
  };
  
  &.loading {
    opacity: 0.5;
    pointer-events: none;
  }
}

.card {
  background: color.side-3();
  position: relative;
  box-shadow: .2em .2em .5em color.side-3(.5);
  cursor: pointer;
  border-radius: 0.5em;
  animation: fadeIn .3s calc(.05s * var(--delay)) both;
  transition: box-shadow 0.2s, background 0.2s, transform 0.2s;
  
  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  &:hover {
    background: color.side-2();
    box-shadow: .1em .1em .2em color.side-3(.2);
    transform: scale(1.05);
    & .title {
      color: color.main();
    }

    & .avatar,
    & .none-avatar {
      opacity: 1;
    }
  }

  & .title {
    width: max-content;
    color: color.main();
    padding: 1em;
    font-size: 1.5em;
    font-weight: bold;
    transition: color 0.2s;
  }

  & .avatar {
    opacity: 0.8;
  }

  & .avatar,
  & .none-avatar {
    height: 100%;
    object-fit: cover;
    position: absolute;
    right: 0;
    top: 0;
    background: color.side-2(.8);
    pointer-events: none;
    border-radius: 0.5em;
    transition: opacity 0.2s;
    user-select: none;
  }

  &.current {
    background: color.side-3();
    box-shadow: .1em .1em .2em color.side-3();
    & .title {
      color: color.white(.8);
    }
    & .avatar,
    & .none-avatar {
      opacity: 0.2;
    }
  }
}
</style>
