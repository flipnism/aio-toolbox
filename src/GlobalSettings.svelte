<script lang="ts">
  import {
    BaseDirectory,
    exists,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import { onMount } from "svelte";
  import ArrayInputText from "./components/ArrayInputText.svelte";
  import InputText from "./components/InputText.svelte";
  import type { GlobalSetting } from "./global";
  import IconButton from "./components/IconButton.svelte";
  import SettingDialog from "./views/SettingDialog.svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import InputBoolean from "./components/InputBoolean.svelte";

  const config_file = "users/settings.json";
  function writeSettingFile(value: any) {
    writeTextFile(config_file, JSON.stringify(value, null, 2), {
      dir: BaseDirectory.AppData,
    });
  }
  function parseSettingFile() {
    readTextFile(config_file, { dir: BaseDirectory.AppData }).then((result) => {
      setting_datas = JSON.parse(result);
    });
  }
  
  listen("update_config", (result) => {
      parseSettingFile();  
  });
  onMount(() => {
    exists(config_file, { dir: BaseDirectory.AppData }).then((exist) => {
      if (!exist) writeSettingFile([]);
      else parseSettingFile();
    });
  });
  $: {
    console.log(setting_datas);
    //emit("setting-update", { data: setting_datas });
  }

  export let modal_data: GlobalSetting = {
    name: "something",
    type: "string",
    values: [],
  };

  function resetData() {
    modal_data = {
      name: "something",
      type: "string",
      values: [],
    };
  }

  let setting_datas: GlobalSetting[] | any;

  let show_setting_dialog = false;
</script>

<SettingDialog
  {modal_data}
  bind:show_modal={show_setting_dialog}
  on:data={(result) => {
    let data = result.detail;
    setting_datas = setting_datas ? [...setting_datas, data] : [data];
  }}
/>
<div class="p-4 gap-2 flex flex-col h-full overflow-y-auto">
  <div class="flex gap-2 self-end">
    <IconButton
      on:click={() => {
        writeSettingFile(setting_datas);
      }}
      icon="save"
      class="btn-outline btn-success btn-circle"
    />
    <IconButton
      on:click={() => {
        resetData();
        show_setting_dialog = true;
      }}
      icon="add"
      class="btn-error btn-circle"
    />
  </div>
  {#if setting_datas && setting_datas.length > 0}
    {#each setting_datas as setting_data, i}
      <div class="flex flex-row items-center gap-2">
        {#if setting_data.type == "string" || setting_data.type == "number"}
          <InputText
            class="grow"
            bind:title={setting_data.name}
            type={setting_data.type}
            bind:value={setting_data.value}
          />
        {:else if setting_data.type == "bool"}
          <InputBoolean
            bind:title={setting_data.name}
            bind:value={setting_data.value}
          />
        {:else}
          <ArrayInputText
            class="grow"
            bind:title={setting_data.name}
            bind:values={setting_data.values}
          />
        {/if}
        <IconButton
          on:click={() => {
            setting_datas = setting_datas.filter((sd) => sd != setting_data);
          }}
          icon="close"
          class="btn-circle"
        />
      </div>
    {/each}
  {/if}
</div>
