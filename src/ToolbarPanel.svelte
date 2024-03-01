<script lang="ts">
  import "./styles.css";
  import { invoke, tauri } from "@tauri-apps/api";
  import { onDestroy, onMount } from "svelte";
  import IconButton from "./components/IconButton.svelte";
  import {
    LogicalPosition,
    LogicalSize,
    PhysicalPosition,
    appWindow,
  } from "@tauri-apps/api/window";
  import { fade, slide } from "svelte/transition";
  import { quartInOut } from "svelte/easing";
  import { listen } from "@tauri-apps/api/event";
  import { layer_index, color_set, global_settings } from "./lib/store";

  import type { GlobalSetting } from "./global";
  let ontop = false;
  let customScripts: any = [];
  let tooltip_text = "";
  let is_loading = false;
  let page_layer_index = 0;
  let btn_scripts_width = 100;
  let gap = 0;
  layer_index.subscribe((value: any) => (page_layer_index = value));
  function setWindowSize() {
    const l = customScripts.length;
    const iconsize = 22 * l;
    gap = 8 * (l - 1);
    btn_scripts_width = iconsize;
    appWindow.setSize(new LogicalSize(btn_scripts_width + gap + 40, 40));
  }
  onMount(() => {
    invoke("list_customscripts").then((result) => {
      customScripts = result;
      setWindowSize();
    });
  });

  function clickThrought(ignore: boolean) {
    invoke("ignore_cursor_events", { ignore: ignore }).then((result) => {
      console.log("result");
    });
  }

  function handleScriptOnClick(script: any) {
    invoke("execute_script", { scriptName: script[1].name });
  }
  let show_script_buttons = false;
  let hideTimerId: number;
  let mouse_down = false;
  let last_position = {
    x: 100,
    y: 100,
  };

  $: appWindow.setAlwaysOnTop(ontop);

  let mouseDown = false;
  let setting_listener: any;
  onMount(async () => {
    setting_listener = await listen<any>("setting-update", (event) => {
      console.log(event.payload);
    });
  });

  const debounce = <T extends (...args: any[]) => void>(fn: T, ms = 0) => {
    let timeoutId: ReturnType<typeof setTimeout>;

    return function (this: ThisParameterType<T>, ...args: Parameters<T>) {
      clearTimeout(timeoutId);
      timeoutId = setTimeout(() => fn.apply(this, args), ms);
    };
  };

  // Your original mouseleave logic
  const handleMouseLeave = () => {
    tooltip_text = "";
    show_script_buttons = false;
    appWindow.setSize(new LogicalSize(58, 40));
  };
  let delayedMouseLeave: any;
  const handleMouseEnter = () => {
    console.log("mouseenter");
    show_script_buttons = true;
    appWindow.setSize(new LogicalSize(btn_scripts_width + gap + 40, 40));

    clearTimeout(delayedMouseLeave);
  };
  onMount(() => {
    delayedMouseLeave = debounce(handleMouseLeave, 500);
  });
  function check(e) {
    console.log(e.type);
  }
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="flex flex-col overflow-hidden select-none">
  <div class="flex align-middle overflow-hidden rounded-sm relative">
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
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
      style="background:{color_set[page_layer_index][0]};color:{color_set[
        page_layer_index
      ][1]}"
      class="flex max-w-8 w-full h-[26px]"
    >
      <div
        data-tauri-drag-region
        class="text-2xl h-full bg-base-300/30 select-none w-[8px] justify-center flex-1 leading-[20px] font-extralight align-middle text-center cursor-grabbing"
      >
        {page_layer_index + 1}
      </div>
    </div>
    <div
      on:mouseleave={delayedMouseLeave}
      on:mouseenter={handleMouseEnter}
      class="flex flex-row"
    >
      <div
        style="background:{color_set[page_layer_index][0]};color:{color_set[
          page_layer_index
        ][1]}"
        class="hello_"
      >
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
          on:mouseenter={() => {}}
          class="min-h-full justify-center items-center flex scale-75 h-full"
        >
          <input
            bind:checked={ontop}
            type="checkbox"
            class="toggle toggle-xs bg-transparent"
          />
        </div>
      </div>
      {#if show_script_buttons}
        <div
          in:slide={{
            delay: 100,
            duration: 200,
            axis: "x",
            easing: quartInOut,
          }}
          out:slide={{
            duration: 100,
            axis: "x",
            easing: quartInOut,
          }}
          style="background:{color_set[page_layer_index][0]};color:{color_set[
            page_layer_index
          ][1]}"
          class="flex flex-row gap-1 transition-all"
        >
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
      {/if}
    </div>
  </div>
  {#if tooltip_text !== ""}
    <div
      transition:fade={{ duration: 100 }}
      class="text-[10px] self-end justify-end bg-base-300/50 rounded-lg px-2 text-white w-fit"
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
