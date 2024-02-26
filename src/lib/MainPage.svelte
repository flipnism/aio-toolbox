<script>
  import { onMount } from "svelte";
  import IconButton from "../components/IconButton.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  onMount(() => {});
</script>

<div class="flex flex-row justify-between w-full items-center gap-2 px-2 py-1">
  <div class="region_title">
    <h1 class="font-black uppercase">Hotkeys</h1>
  </div>
  <slot />
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
