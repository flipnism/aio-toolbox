<script lang="ts">
  import "./styles.css";
  import { slide } from "svelte/transition";
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import MainPage from "./lib/MainPage.svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import { quintInOut } from "svelte/easing";
  import IconButton from "./components/IconButton.svelte";
  import { appDataDir } from "@tauri-apps/api/path";
  import { readTextFile } from "@tauri-apps/api/fs";
  import MacroPanel from "./views/MacroPanel.svelte";
  let macropad_data: any = [];
  let action_data: string[] = [];
  let active_index = [-1, -1];
  let active_layer = 0;
  let show_action_list = false;
  let show_macro_panel = false;
  let macro_mode = false;

  onMount(() => {
    listen("mod-event", (e) => {
      console.log(e);
    });
    appDataDir().then((result) => {
      readTextFile(result + "users/action_data.json").then((data) => {
        action_data = JSON.parse(data);
      });
    });
    invoke("get_hotkey_config", { filename: "macropad.json" })
      .then((result) => {
        console.log(result);
        macropad_data = result;
      })
      .catch((e) => console.log(e));
  });
  function active(index: number, idx: number) {
    active_index[0] == index && active_index[1] == idx;
  }
  function is_mod(index: number, idx: number) {
    return index == 2 && idx == 3;
  }
  function active_btn_data() {
    console.log(
      macropad_data[active_layer].data[active_index[0]][active_index[1]]
    );
  }
</script>

<div class="flex flex-col">
  <MainPage>
    <div class="flex flex-cols p-0 -m-2 items-center">
      <p class="text-base-content">
        {active_layer + 1}
      </p>
      <label class="cursor-pointer label gap-1">
        <input
          bind:checked={macro_mode}
          type="checkbox"
          class="toggle toggle-error toggle-xs"
        />
        <span class="label-text">Macro</span>
      </label>
    </div>
  </MainPage>
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
        <div class="flex flex-row flex-wrap gap-2 justify-center">
          {#each action_data as action, i}
            <button
              on:click={() => {
                active_btn_data();
              }}
              class="btn btn-xs btn-outline font-bold uppercase"
              >{action}</button
            >
          {/each}
        </div>
      </div>
    {/if}
    <MacroPanel
      bind:show_macropanel={show_macro_panel}
      class="flex flex-col absolute bg-base-300 z-10 w-full h-[calc(100%-2rem)] p-4"
    />
    <div class="h-full">
      <div class="grid grid-cols-4 gap-4 px-10">
        {#if macropad_data.length > 0}
          {#each macropad_data[active_layer].data as macropads, index}
            {#each macropads as mpads, idx}
              <button
                on:click={() => {
                  if (is_mod(index, idx)) {
                    active_layer = (active_layer + 1) % 3;
                    active_index = [-1, -1];

                    show_action_list = false;

                    return;
                  }
                  active_index = [index, idx];
                  if (macro_mode) show_macro_panel = true;
                  else show_action_list = true;
                }}
                class=" border-[2px] bg-base-300 {active_index[0] == index &&
                active_index[1] == idx
                  ? 'border-error/90'
                  : 'border-error/0'} rounded-xl"
              >
                <div
                  class="bg-opacity-90 cursor-pointer box-border border-b-base-300 border-b-[15px] border-x-[8px] border-t-4 border-t-base-content/30 border-r-base-content/20 border-5-base-content/10 border-l-base-content/10 p-2 rounded-[.5rem] {mpads.key ==
                  'F24'
                    ? 'bg-base-content/20'
                    : 'bg-base-200'} select-none h-[100px] drop-shadow-[0_10px_5px_rgba(0,0,0,.4)] active:border-b-[10px] active:drop-shadow-[0_2px_5px_rgba(0,0,0,.4)]"
                >
                  <div class="flex flex-row justify-between">
                    <p class="text-left font-black text-4xl">{mpads.key}</p>
                    <span class="text-xs italic">{mpads?.data_mode || ""}</span>
                  </div>
                  <span class="text-xs">{mpads?.data?.data || ""}</span>
                </div>
              </button>
            {/each}
          {/each}
        {/if}
      </div>

      <div class="flex flex-row absolute w-full bottom-0 left-0">
        <div
          class="h-2 {active_layer == 0 ? 'bg-error' : 'bg-base-300'} flex-1"
        ></div>
        <div
          class="h-2 {active_layer == 1 ? 'bg-error' : 'bg-base-300'} flex-1"
        ></div>
        <div
          class="h-2 {active_layer == 2 ? 'bg-error' : 'bg-base-300'} flex-1"
        ></div>
      </div>
    </div>
  </div>
</div>
