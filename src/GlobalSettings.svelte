<script lang="ts">
  import {
    BaseDirectory,
    exists,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import { onMount } from "svelte";
  import { tags } from "./lib/store";
  import { fade } from "svelte/transition";
  import ArrayInputText from "./components/ArrayInputText.svelte";

  const config_file = "users/settings.json";
  function writeDefaultFile() {
    writeTextFile(config_file, JSON.stringify([], null, 2), {
      dir: BaseDirectory.AppData,
    });
  }
  onMount(() => {
    exists(config_file, { dir: BaseDirectory.AppData }).then((exist) => {
      if (!exist) writeDefaultFile();
    });
  });
</script>

<div class="p-4 gap-2 flex flex-col">
  <ArrayInputText title="Something!" />
  <ArrayInputText title="Something Else!" />
</div>
