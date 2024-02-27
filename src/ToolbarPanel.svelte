<script lang="ts">
  import "./styles.css";
  import { invoke } from "@tauri-apps/api";
  import { onMount } from "svelte";
  import IconButton from "./components/IconButton.svelte";
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  import { fade, slide } from "svelte/transition";
  import { quartInOut } from "svelte/easing";
  import { listen } from "@tauri-apps/api/event";
  import { layer_index, color_set } from "./lib/store";
  let ontop = false;
  let customScripts: any = [];
  let tooltip_text = "";
  let is_loading = false;
  let page_layer_index = 0;
  layer_index.subscribe((value: any) => (page_layer_index = value));

  function setWindowSize() {
    const l = customScripts.length;
    const iconsize = 22 * l;
    const gap = 8 * (l - 1);
    appWindow.setSize(new LogicalSize(iconsize + gap + 40, 40));
  }
  onMount(() => {
    invoke("list_customscripts").then((result) => {
      customScripts = result;
      setWindowSize();
    });
  });

  listen("socket_message", (message) => {
    console.log(message.payload);
  });

  function handleScriptOnClick(script: any) {
    invoke("execute_script", { scriptName: script[1].name });
  }
</script>

<div
  on:mouseleave={() => (tooltip_text = "")}
  class="flex flex-col overflow-hidden"
>
  <div
    style="background:{color_set[page_layer_index][0]};color:{color_set[
      page_layer_index
    ][1]}"
    class="flex align-middle overflow-hidden rounded-sm border-[1px] border-white/20 relative"
  >
    {#if is_loading}
      <div
        style="background:{color_set[0][0]};"
        transition:slide={{ axis: "x", duration: 200, easing: quartInOut }}
        data-tauri-drag-region
        class="flex rounded-md cursor-grabbing h-full absolute top-0 left-0 w-full justify-center items-center"
      >
        <span class="loading loading-dots loading-xs"></span>
      </div>
    {/if}
    <div class="flex flex-row gap-1">
      {#each customScripts as script, i}
        <IconButton
          on:mouseover={() => {
            tooltip_text = script[1].desc;
          }}
          on:click={() => handleScriptOnClick(script)}
          icon="path"
          path={script[1].icon_path}
        />
      {/each}
    </div>
    <div class="min-h-full justify-center items-center flex scale-75">
      <input
        bind:checked={ontop}
        type="checkbox"
        class="toggle toggle-xs bg-transparent"
      />
    </div>
    <div
      data-tauri-drag-region
      class="text-2xl bg-base-300/30 select-none w-[8px] justify-center flex-1 leading-[20px] font-extralight align-middle text-center cursor-grabbing"
    >
      {page_layer_index + 1}
    </div>
  </div>
  {#if tooltip_text !== ""}
    <div
      transition:fade={{ duration: 100 }}
      class="text-[10px] justify-end bg-base-300/50 rounded-lg px-2 text-white w-fit"
    >
      {tooltip_text}
    </div>
  {/if}
</div>

<style>
  :global(html),
  :global(:root) {
    background: transparent !important;
  }
</style>
