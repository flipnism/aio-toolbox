<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import "./styles.css";
  import { listen } from "@tauri-apps/api/event";
  import { BaseDirectory, readTextFile } from "@tauri-apps/api/fs";
  import {  onMount } from "svelte";
  import {
    LogicalPosition,
    LogicalSize,
    appWindow,
  } from "@tauri-apps/api/window";
  import CustomScriptButton from "./componentsv2/CustomScriptButton.svelte";
  
  import type { AppConfig } from "./global";
  let keys: any = null;
  let mod_pressed = { pressed: false, milisecond: 0 };
  let configs: any = null;
  let filtered_config: any[] | null = null;
  let progressId: number;
  let delayIdle: number;
  let opacity = 1;
  let progress = 0;
  let customScripts = [];
  let app_configs:AppConfig;
  let mod_pressed_count = 0;
  
  const doIdle = () => {
    opacity = 0;
    clearTimeout(delayIdle);
  };
  const dontIdle = () => {
    opacity = 1;
    clearTimeout(delayIdle);
    counter = app_configs.hide_delay / 1000;
    doCountDown();
    delayIdle = setTimeout(() => {
      doIdle();
    }, app_configs.hide_delay);
  };
  let doubleId: number;
  const doublePressMod = () => {
    clearTimeout(doubleId);
    doubleId = setTimeout(() => {
      mod_pressed_count = 0;
    }, 200);
    mod_pressed_count += 1;
  };
  let current_pos =0;
  $: {
    if (app_configs){
      const p = app_configs.window_position[current_pos];
      console.log(p);
      appWindow.setPosition(new LogicalPosition(p[0],p[1]))
  }
  }
  let mouse_location:[number,number]=[0,0];
  
   function setPanelPosition() {
    if (!app_configs.follow_cursor) return;
    if (mod_pressed_count <= 5) {
      doublePressMod();
      return;
    }
    invoke("mouse_location").then((result) => {
      mouse_location = result as [number,number];
      current_pos = (current_pos+1)%app_configs.window_position.length;
      console.log(current_pos);
//      appWindow.setPosition(new LogicalPosition(x, y));
      mod_pressed_count = 0;
    });
  }
  listen("update_config", (result) => {
    loadAppConfig();
  });

  listen("modkey_event", (result) => {
    dontIdle();
    keys = result.payload;
    if (keys.config_updated) {
      location.reload();
    }
    if (keys.reset) {
      keys = { ...keys, mod_pressed: false };

      doFilter(configs);
      return;
    }
    if (!keys.multikey && !keys.mod_pressed) {
      doFilter(configs);
      return;
    }

    if (
      keys.multikey &&
      keys.mod_pressed &&
      keys.key_1 == "" &&
      keys.key_2 == ""
    ) {
      let key_map = keys.multikey_map.map((item) => {
        return {
          key_name: item,
          key_desc: "",
          key_mode: "",
          key_multikey: false,
          key_1: "",
          key_2: "",
        };
      });
      filtered_config = pads.map((padKey) => {
        const matchingData = key_map.find((item) => item.key_name == padKey);
        return matchingData ? matchingData : null;
      });
      return;
    }

    if (keys.key_1) {
      invoke("filter_keys", { key: keys.key_1 }).then((result) => {
        if (keys.reset) {
          keys = {
            reset: true,
            mod_pressed: false,
            key_1: "",
            key_2: "",
          };
          return;
        }
        filtered_config = pads.map((padKey) => {
          const matchingData = result.find((item) => item.key_2 == padKey);
          return matchingData ? matchingData : null;
        });

        return;
      });
    } else {
      filtered_config = [];
    }
    if (keys.mod_pressed) return;

    doFilter(configs);
  });
  function processProgress() {
    progress += 5;
  }
  listen("mod_pressed", (r) => {
    mod_pressed = r.payload;
    setPanelPosition();
    if (mod_pressed.pressed) {
      progressId = setInterval(processProgress, 5);
    } else {
      clearInterval(progressId);
      progress = 0;
    }
  });
  let countdown: number;
  function doCountDown() {
    if (countdown) clearInterval(countdown);
    countdown = setInterval(() => {
      counter -= 1;
      if (counter <= 0) {
        counter = 0;
        clearInterval(countdown);
      }
    }, 1000);
  }

  function loadAppConfig() {
    readTextFile("app_config.json", { dir: BaseDirectory.AppData }).then(
      (result) => {
        app_configs = JSON.parse(result);
        counter = app_configs.hide_delay / 1000;
  
        doCountDown();
      }
    );
  }

  onMount(() => {
    invoke("script_lists").then((result) => {});
    invoke("list_customscripts").then((result) => {
      customScripts = result;
    });
    loadAppConfig();
    readTextFile("config.json", { dir: BaseDirectory.AppData }).then(
      (result) => {
        configs = JSON.parse(result);
        keys = {
          reset: true,
          mod_pressed: false,
          key_1: "",
          key_2: "",
        };
        doFilter(configs);
      }
    );
    appWindow.setSize(new LogicalSize(400, 150));
  });
  let current_desc = "";
  let pads = [
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "F24",
  ];
  let counter = 0;
  function doFilter(config) {
    const filt = config.filter((e) => !e.key_multikey == !keys.mod_pressed);
    filtered_config = pads.map((padKey) => {
      const matchingData = filt.find((item) => item.key_1 == padKey);

      return matchingData ? matchingData : null;
    });
  }

  /**
   * Description
   * @param {any} k
   * @returns {any}
   */
  function key(k) {
    return `Combo: ${k.key_1}${k.key_2 != "" ? "-" + k.key_2 : " _"}`;
  }

  function whichkey(key) {
    let is_result = false;
    if (keys.multikey) {
      is_result = keys.key_2 == key;
    } else {
      is_result = keys.key_1 == key;
    }

    return is_result;
  }
  function handleScriptOnClick(script) {
    invoke("execute_script", { scriptName: script[1].name });
  }
  function findIcon(keyname: String) {
    let v = null;
    customScripts.find((script) => {
      if (script[0] == keyname) {
        /**
         * name
         * desc
         * icon_path
         * executable
         *
         */
        v = script[1];
      }
    });
    return v;
  }
</script>

<!-- svelte-ignore avoid-mouse-events-on-document -->
<svelte:document on:mouseenter={dontIdle} />
{#if app_configs}
<div style="opacity: {opacity};" class="transition-all bg-base-300/90">
  <div
    class="absolute top-0 left-0 pointer-events-none overflow-hidden h-full w-full z-50"
  >
      <div class="text-center text-[.7rem] font-black p-[5px]">{mouse_location}</div>
    <div
      style="width:{progress * 1}px;height:{progress * 1}px;opacity:{progress /
        5 /
        100};"
      class="{progress >= app_configs.hold_delay
        ? progress >= app_configs.reset_delay
          ? 'bg-error transition-all'
          : 'bg-success'
        : 'bg-white'}  w-10 h-10 rounded-full top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 relative"
    ></div>
  </div>

  <div class="flex overflow-hidden flex-col select-none h-screen w-screen">
    <div class="flex items-center">
      <div class="text-[.7rem] font-black p-[5px]">{counter}</div>
      {#if keys}
        {#if keys.mod_pressed}
          <div class="text-[.7rem] font-black p-[5px]">
            {key(keys)}
          </div>
        {:else}
          <div class="text-[.7rem] p-[5px] font-black">
            Key: {keys.key_1 || "_"}
          </div>
        {/if}
      {/if}
      <div
        data-tauri-drag-region
        class="h-4/6 flex-1 cursor-grabbing hover:bg-base-100/20 active:bg-base-100/20 rounded-lg"
      ></div>
      <div
        class="font-light text-base-content/50 line-clamp-1 px-2 text-[.6rem]"
      >
        {current_desc}
      </div>
    </div>

    {#if filtered_config && filtered_config.filter((e) => e != null).length > 0}
      {#key filtered_config}
        <div class="grid grid-cols-4 overflow-hidden">
          {#each filtered_config as key, i}
            <!-- svelte-ignore a11y-no-static-element-interactions -->

            <div
              on:mouseenter={() => {
                if (key) current_desc = key.key_desc;
              }}
              on:mouseleave={() => {
                current_desc = "";
              }}
              class="border min-h-10 {key != null && key.key_multikey
                ? 'bg-success/5 border-white/5'
                : whichkey(pads[i])
                  ? 'border-warning border-2'
                  : 'border-white/5'} text-xs p-1"
            >
              {#if key}
                {#if key.key_mode == "CustomScript"}
                  <CustomScriptButton
                    key={pads[i]}
                    obj={findIcon(key.key_data.CustomScript.data)}
                  />
                {:else}
                  <div
                    class="flex w-full text-[.7rem] uppercase {key.key_mode !=
                    ''
                      ? ''
                      : 'bg-success/20'}"
                  >
                    <div
                      class="font-black {key.key_mode != ''
                        ? ''
                        : 'text-xl text-center self-center w-full'}"
                    >
                      {key.key_name}
                    </div>
                    <div class="font-extralight text-base-content/50"></div>
                    <div class="flex-1"></div>
                    {#if key.key_mode != ""}
                      <div
                        class="font-black italic {key.key_mode == 'Action'
                          ? 'bg-success'
                          : 'bg-error'} rounded-full w-4 h-4 text-center text-base-300"
                      >
                        {key.key_mode.slice(0, 1)}
                      </div>
                    {/if}
                  </div>
                  <div class="font-light line-clamp-1 text-[.5rem]">
                    {key.key_mode != "" ? pads[i] : ""}
                  </div>
                {/if}
              {/if}
            </div>
          {/each}
        </div>
      {/key}
    {:else}
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div
        on:mouseenter={() => {
          current_desc = "";
        }}
        class="flex text-center h-screen w-screen justify-center items-center text-4xl text-white/10 absolute"
      >
        None
      </div>
    {/if}
  </div>
</div>
{/if}
<style>
  :global(html),
  :global(:root) {
    font-family: "JetBrains Mono";
    background: transparent !important;
  }
</style>
