<script lang="ts">
  import { fade } from "svelte/transition";
  import EditableP from "./EditableP.svelte";

  export let title = "Title here:";

  export let value: any = "";
  export let type = "string";
  let idx = 0;
</script>

<div class="flex flex-row {$$props.class}">
  <EditableP class="font-black px-2 w-1/3 self-center" bind:value={title} />
  <div
    class="flex w-full bg-base-200 rounded-lg flex-row gap-1 flex-wrap items-center p-2"
  >
    {#if type == "string"}
      <input
        tabindex="0"
        bind:value
        type="text"
        class="grow input input-xs input-ghost border-none focus:outline-none"
        placeholder={"type something"}
      />
    {:else}
      <input
        tabindex="0"
        bind:value
        type="number"
        class="grow input input-xs input-ghost border-none focus:outline-none"
        placeholder={"type something"}
      />
    {/if}
  </div>
</div>
