<script lang="ts">
  import { invoke } from "@tauri-apps/api";
  import {
    BaseDirectory,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import type { AppConfig } from "./global";
    import { listen } from "@tauri-apps/api/event";

  let app_config: AppConfig;
   function loadAppConfig(){
    
    readTextFile("app_config.json", { dir: BaseDirectory.AppData }).then(
      (result) => {
        app_config = JSON.parse(result);
      },
    );
  }
   onMount(() => {
    loadAppConfig();
  });
  listen("update_config",(result)=>{
    loadAppConfig();
  })
  function saveConfig() {
    writeTextFile("app_config.json", JSON.stringify(app_config, null, 2), {
      dir: BaseDirectory.AppData,
    })
      .then(async (result) => {
        await invoke("update_config").catch((e) => console.log(e));
      })
      .catch((e) => console.log(e));
  }
  // $: {
  //   writeTextFile("app_config.json", JSON.stringify(app_config, null, 2), {
  //     dir: BaseDirectory.AppData,
  //   })
  //     .then(async (result) => {
  //       await invoke("update_config").catch((e) => console.log(e));
  //     })
  //     .catch((e) => console.log(e));
  // }
  function onWindowPositionChange(e: Event) {
    const target = e.target as HTMLInputElement;
    const result = target.value;
    const values = JSON.parse(result);
    app_config.window_position = values;
  }
</script>

<div in:fade={{ delay: 100 }} out:fade={{ duration: 100 }} class="">
  {#if app_config}
    <table class="table table-zebra table-sm table-fixed">
      <thead class="bg-base-300 font-black text-lg">
        <th>
          <button on:click={saveConfig} class="btn btn-xs btn-neutral"
            >Save Config</button
          >
        </th>
        <th></th>
      </thead>
      <tbody>
        <tr>
          <td>Click Throught</td>
          <td>
            <input
              type="checkbox"
              class="toggle toggle-error toggle-xs"
              bind:checked={app_config.click_throught}
            />
          </td>
        </tr>
        <tr>
          <td>Always on Top</td>
          <td>
            <input
              type="checkbox"
              class="toggle toggle-error toggle-xs"
              bind:checked={app_config.always_on_top}
            />
          </td>
        </tr>
        <tr>
          <td>Follow Cursor</td>
          <td>
            <input
              type="checkbox"
              class="toggle toggle-error toggle-xs"
              bind:checked={app_config.follow_cursor}
            />
          </td>
        </tr>
        <tr>
          <td>Hold Delay</td>
          <td>
            <input
              type="number"
              class="input input-sm input-ghost"
              bind:value={app_config.hold_delay}
            />
          </td>
        </tr>
        <tr>
          <td>Reset Delay</td>
          <td>
            <input
              type="number"
              class="input input-sm input-ghost"
              bind:value={app_config.reset_delay}
            />
          </td>
        </tr>
        <tr>
          <td>Hide Delay</td>
          <td>
            <input
              type="number"
              class="input input-sm input-ghost"
              bind:value={app_config.hide_delay}
            />
          </td>
        </tr>

        <tr>
          <td>Window Position</td>
          <td>
            <input
              type="text"
              class="input input-sm input-ghost"
              value={JSON.stringify(app_config.window_position)}
              on:change={onWindowPositionChange}
            />
          </td>
        </tr>
      </tbody>
    </table>
  {/if}
</div>
