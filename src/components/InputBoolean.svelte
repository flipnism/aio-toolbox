<script lang="ts">
  import { fade } from "svelte/transition";
  import EditableP from "./EditableP.svelte";

  export let title = "Title here:";
  export let value = false;
  let idx = 0;
</script>

<div class="flex flex-row w-full">
  <EditableP class="font-black px-2 w-1/3 self-center" value={title} />
  <div
    class="flex w-full bg-base-200 rounded-lg flex-row gap-1 flex-wrap items-center p-2"
  >
    <div class="min-w-full">
      <input
        tabindex="0"
        type="checkbox"
        bind:checked={value}
        class="toggle toggle-error toggle-xs grow"
      />
    </div>
  </div>
</div>
