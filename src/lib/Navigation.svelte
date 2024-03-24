<script lang="ts">
  import { onMount } from "svelte";
  import IconButton from "../components/IconButton.svelte";
  import { appWindow } from "@tauri-apps/api/window";
  import { fade, slide } from "svelte/transition";
  import {
    backIn,
    backOut,
    bounceIn,
    quadOut,
    quintInOut,
  } from "svelte/easing";

  export let page_state = 2;
  let menus = ["Hotkeys", "Settings", "TodoList"];
  function handleOnClick(text: string) {
    page_state = menus.findIndex((m) => m == text); //(page_state + 1) % menus.length;
  }
  let show_menu = true;
  let always_on_top = false;
  $: appWindow.setAlwaysOnTop(always_on_top);
</script>

<div
  class="flex flex-row justify-between w-full items-center gap-2 px-2 py-1 select-none"
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    on:mouseenter={() => (show_menu = true)}
    on:mouseleave={() => (show_menu = true)}
    class="region_title flex"
  >
    <!--
    <div class="font-black uppercase select-none px-2">
      {menus[page_state]}
    </div>
    -->
    {#if show_menu}
      <div
        in:slide={{ axis: "x", duration: 300, easing: backIn }}
        out:slide={{ axis: "x", duration: 300, delay: 300, easing: backOut }}
        class="flex flex-row"
      >
        {#each menus as menu, i}
          <button
            tabindex="-1"
            on:click={() => handleOnClick(menu)}
            class="text-md tracking-[-.02rem] uppercase select-none btn btn-xs btn-ghost {menus[page_state]==menu?"font-black":"font-light text-base-content/20"}"
          >
            {menu}
          </button>
        {/each}
      </div>
    {/if}
  </div>
  <slot name="title" />
  <div
    data-tauri-drag-region
    class="h-5 grow bg-base-200 rounded-lg cursor-grabbing"
  ></div>
  <div class="btn_region flex self-end items-center gap-2">
    <input
      bind:checked={always_on_top}
      type="checkbox"
      class="toggle toggle-xs toggle-error scale-75"
    />
    <div class="w-2"></div>
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
