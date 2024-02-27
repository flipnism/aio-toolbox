<script>
  import { onMount } from "svelte";
  import IconButton from "../components/IconButton.svelte";
  import { appWindow } from "@tauri-apps/api/window";

  export let page_state = 0;
  let menus = ["Hotkeys", "Settings"];
  function handleOnClick() {
    page_state = (page_state + 1) % menus.length;
  }
</script>

<div
  class="flex flex-row justify-between w-full items-center gap-2 px-2 py-1 select-none"
>
  <div class="region_title">
    <button
      tabindex="-1"
      on:click={handleOnClick}
      class="font-black uppercase select-none btn btn-xs"
      >{menus[page_state]}</button
    >
  </div>
  <slot name="title" />
  <div
    data-tauri-drag-region
    class="h-5 grow bg-base-200 rounded-lg cursor-grabbing"
  ></div>
  <div class="btn_region flex self-end gap-2">
    <IconButton
      on:click={() => {
        appWindow.minimize();
      }}
      icon="minimize"
    />
    <IconButton
      on:click={() => {
        appWindow.close();
      }}
      icon="close"
    />
  </div>
</div>
<slot />
