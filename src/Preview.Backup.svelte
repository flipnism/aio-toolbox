<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import "./styles.css";
  import { onMount } from "svelte";
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
  import KeyPreview from "./views/KeyPreview.svelte";
  import IconButton from "./components/IconButton.svelte";
  import { fade } from "svelte/transition";
  import { layer_index, color_set } from "./lib/store";

  let current_data: any = [];
  let macropad_lists: any = [];

  let page_layer_index = 0;
  layer_index.subscribe((value) => (page_layer_index = value));
  onMount(async () => {
    const contents = await readTextFile("users/macropad.json", {
      dir: BaseDirectory.AppData,
    });
    macropad_lists = JSON.parse(contents);
    current_data = macropad_lists[page_layer_index].data.flat();
    appWindow.setSize(new LogicalSize(350, 90));
  });
</script>

{#if current_data.length > 0}
  <div
    transition:fade
    class="grid grid-cols-4 h-screen cursor-grab"
    data-tauri-drag-region
  >
    <IconButton
      on:click={() => appWindow.close()}
      style="color:{color_set[page_layer_index][1]};"
      icon="close"
      class={`absolute top-0 right-0 btn btn-circle btn-xs btn-ghost text-white`}
    />
    {#if current_data}
      {#each current_data as list, i}
        <KeyPreview
          value={list}
          is_mod={current_data.length - 1 == i}
          which_layer={page_layer_index}
        />
      {/each}
    {/if}
  </div>
{/if}
