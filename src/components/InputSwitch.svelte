<script lang="ts">
  import { fade } from "svelte/transition";
  import EditableP from "./EditableP.svelte";

  export let title = "Title here:";

  export let options = [""];
  export let value: any = "";
  let idx = 0;
</script>

<div class="flex flex-row">
  <EditableP class="font-black px-2 w-1/3 self-center" value={title} />
  <div
    class="flex w-full bg-base-200 rounded-lg flex-row gap-1 flex-wrap items-center p-2"
  >
    <button
      on:click={() => {
        idx = (idx + 1) % options.length;
        value = options[idx];
      }}
      tabindex="0"
      class="btn btn-xs text-left"
      placeholder={"type something"}
    >
      {options[idx]}
    </button>
  </div>
</div>
