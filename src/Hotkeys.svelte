<script lang="ts">
  import "./styles.css";
  import { fade, slide } from "svelte/transition";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { quintInOut } from "svelte/easing";
  import IconButton from "./components/IconButton.svelte";
  import { appDataDir } from "@tauri-apps/api/path";
  import { readTextFile } from "@tauri-apps/api/fs";
  import MacroPanel from "./views/MacroPanel.svelte";
  import { LogicalSize, appWindow } from "@tauri-apps/api/window";
  import { layer_index } from "./lib/store";

  let macropad_data: any = [];
  let action_data: string[] = [];
  let active_index = [-1, -1];
  let show_action_list = false;
  let show_macro_panel = false;
  let macro_mode = false;
  let page_layer_index = 0;
  let mouse_pos = [0, 0];
  let show_tooltip = false;
  let tooltip = null;

  layer_index.subscribe((value: any) => (page_layer_index = value));

  onMount(() => {
    appDataDir().then((result) => {
      readTextFile(result + "users/action_data.json").then((data) => {
        action_data = JSON.parse(data);
      });
    });
    invoke("get_hotkey_config", { filename: "macropad.json" })
      .then((result) => {
        macropad_data = result;
      })
      .catch((e) => console.log(e));
    appWindow.setSize(new LogicalSize(750, 500));
  });
  function active(index: number, idx: number) {
    active_index[0] == index && active_index[1] == idx;
  }
  function is_mod(index: number, idx: number) {
    return index == 2 && idx == 3;
  }
  function active_btn_data() {
    console.log(
      macropad_data[page_layer_index].data[active_index[0]][active_index[1]]
    );
  }
  function stringifyKBD(data_macro: any) {
    return data_macro
      .filter((e: any) => e.down)
      .map((e: any) => e.key)
      .join(" ")
      .toUpperCase();
  }
  function reset() {
    show_tooltip = false;
    // active_index = [-1, -1];
    // show_action_list = false;
    // show_macro_panel = false;
    setTimeout(() => {
      if (!show_macro_panel && !show_action_list) {
        active_index = [-1, -1];
      }
    }, 300);
  }

  function handleButtonClick(index: number, idx: number) {
    if (is_mod(index, idx)) {
      layer_index.set((page_layer_index + 1) % 3);
      active_index = [-1, -1];
      show_action_list = false;

      return;
    }
    active_index = [index, idx];
    if (macro_mode) {
      show_macro_panel = true;
      show_action_list = false;
    } else show_action_list = true;
  }
</script>

<svelte:document
  on:contextmenu|preventDefault={(e) => {
    show_tooltip = true;
  }}
/>
<div
  out:fade={{ duration: 200 }}
  in:fade={{ duration: 200, delay: 200 }}
  class="flex flex-col gap-2"
>
  <div class="flex justify-end flex-cols p-0 items-center px-12 gap-2">
    <label class="cursor-pointer label gap-1 justify-center items-center">
      <span class="label-text text-xs italic">Macro mode</span>
      <input
        bind:checked={macro_mode}
        type="checkbox"
        class="toggle toggle-error toggle-xs"
      />
    </label>
    <button class="btn btn-circle btn-xs btn-outline font-black">
      {page_layer_index + 1}
    </button>
  </div>
  <div class="flex flex-col">
    {#if show_tooltip}
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <divs
        transition:fade={{ duration: 100 }}
        on:mouseleave={reset}
        bind:this={tooltip}
        style="top: {mouse_pos[1]}px;left:{mouse_pos[0]}px;"
        class="absolute flex gap-1 p-1 z-20 bg-error text-error-content drop-shadow-lg rounded-full"
      >
        <IconButton
          icon="action"
          on:click={() => {
            show_action_list = true;
            show_tooltip = false;
          }}
        />
        <IconButton
          icon="macro"
          on:click={() => {
            macro_mode = true;
            show_macro_panel = true;
            show_action_list = false;
            show_tooltip = false;
          }}
        />
      </divs>
    {/if}
    <div class="">
      {#if show_action_list}
        <div
          transition:slide={{ axis: "y", duration: 200, easing: quintInOut }}
          class="z-10 absolute w-full bottom-0 h-1/3 bg-base-300 p-4 drop-shadow-[0_-5px_10px_rgb(0,0,0,.2)]"
        >
          <IconButton
            on:click={() => (show_action_list = false)}
            icon="close"
            class="btn-error btn-circle absolute top-2 right-2"
          />
          <div
            class="flex flex-row flex-wrap gap-2 py-5 items-center h-full justify-center"
          >
            {#each action_data as action, i}
              <button
                on:click={() => {
                  active_btn_data();
                }}
                class="btn btn-xs btn-outline font-bold uppercase rounded-sm"
              >
                {action}
              </button>
            {/each}
          </div>
        </div>
      {/if}
      <MacroPanel
        bind:show_macropanel={show_macro_panel}
        class="flex flex-col absolute bg-base-300 z-10 w-full h-[calc(100%-2rem)] p-4"
      />
      <div class="h-full">
        <div class="grid grid-cols-4 gap-1 px-4">
          {#if macropad_data.length > 0}
            {#each macropad_data[page_layer_index].data as macropads, index}
              {#each macropads as mpads, idx}
                <button
                  on:contextmenu={(e) => {
                    if (is_mod(index, idx)) return;
                    active_index = [index, idx];
                    mouse_pos = [e.clientX, e.clientY];
                  }}
                  on:click={() => {
                    handleButtonClick(index, idx);
                  }}
                  class=" border-[2px] bg-base-300 {active_index[0] == index &&
                  active_index[1] == idx
                    ? 'border-error/90'
                    : 'border-error/0'} rounded-xl"
                >
                  <div
                    class="bg-opacity-90 flex flex-col gap-1 cursor-pointer box-border border-b-base-300 border-b-[15px] border-x-[8px] border-t-4 border-t-base-content/30 border-r-base-content/20 border-5-base-content/10 border-l-base-content/10 p-2 rounded-[.5rem] {mpads.key ==
                    'F24'
                      ? 'bg-base-content/20'
                      : 'bg-base-200'} select-none h-[100px] drop-shadow-[0_10px_5px_rgba(0,0,0,.4)] active:border-b-[10px] active:drop-shadow-[0_2px_5px_rgba(0,0,0,.4)]"
                  >
                    <div class="flex flex-row justify-between gap-1">
                      <p class="text-left font-black text-4xl">
                        {mpads.key
                          .replace(/^Numpad/g, "N")
                          .replace(/Insert/g, "INS")}
                      </p>
                      <div class="flex flex-col items-end">
                        <span
                          class="{mpads?.data_mode == 'macro'
                            ? 'text-error'
                            : ''} text-xs font-bold italic"
                        >
                          {mpads?.data_mode || ""}
                        </span>
                        {#if mpads?.data_mode == "macro"}
                          <span class="text-xs line-clamp-1">
                            {mpads?.macro_title}
                          </span>
                        {/if}
                      </div>
                    </div>
                    {#if mpads?.data_mode == "macro"}
                      <div
                        class="text-[.5rem] border-[1px] border-neutral/50 p-[2px]"
                      >
                        {stringifyKBD(mpads.data_macro)}
                      </div>
                    {:else}
                      <span class="text-xs">{mpads?.data?.data || ""}</span>
                    {/if}
                  </div>
                </button>
              {/each}
            {/each}
          {/if}
        </div>

        <div class="flex flex-row absolute w-full bottom-0 left-0">
          <div
            class="h-2 {page_layer_index == 0
              ? 'bg-error'
              : 'bg-base-300'} flex-1"
          ></div>
          <div
            class="h-2 {page_layer_index == 1
              ? 'bg-error'
              : 'bg-base-300'} flex-1"
          ></div>
          <div
            class="h-2 {page_layer_index == 2
              ? 'bg-error'
              : 'bg-base-300'} flex-1"
          ></div>
        </div>
      </div>
    </div>
  </div>
</div>
