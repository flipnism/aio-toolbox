<script lang="ts">
  import type { Item } from "../global";
  import { tags, writable } from "../lib/store";
  import { fade } from "svelte/transition";
  import EditableP from "./EditableP.svelte";
  import IconButton from "./IconButton.svelte";

  export let title = "Title here:";
  const local_tags = writable([]);
  export let values: Item[] | [] = [];
  local_tags.subscribe((tagitem: Item[]) => (tagitem = values));

  let text = "";
  let idx = 0;
  function incrementIndex() {
    idx = idx + 1;
  }
  function addItem(e: any) {
    if (e.key == "Backspace") {
      const data = values[values.length - 1];
      if (text !== "") return;
      removeItem(data.id);
      return;
    }
    if (e.key == "Enter") {
      if (e.target.value.trim() == "") return;

      incrementIndex();

      values = [...values, { id: idx, value: e.target.value }];
      console.log(values);

      text = "";
    }
  }

  function removeItem(id: number) {
    values = values.filter((tag) => tag.id != id);
    console.log(values);
  }
  function onFocusOut(e: any) {
    if (e.target.value === "") return;
    incrementIndex();
    values = [...values, { id: idx, value: e.target.value }];
    console.log(values);

    text = "";
  }
</script>

<div class="flex flex-row {$$props.class}">
  <EditableP class="font-black px-2 w-1/3 self-center" bind:value={title} />
  <div
    class="flex w-full bg-base-200 rounded-lg flex-row gap-1 flex-wrap items-center p-2"
  >
    {#if values}
      {#each values as tag, i}
        <button
          on:click={() => {
            removeItem(tag.id);
          }}
          tabindex="-1"
          class="btn btn-xs text-[10px] p-1 m-0 btn-outline rounded-sm shrink justify-between text-base-content border-opacity-20 bg-opacity-10"
        >
          {tag.value}
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="h-3 w-3"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            ><path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            /></svg
          >
        </button>
      {/each}
    {/if}

    <input
      on:focusout={onFocusOut}
      tabindex="0"
      bind:value={text}
      on:keydown={addItem}
      type="text"
      class="grow input input-xs input-ghost border-none focus:outline-none"
      placeholder={values.length <= 0 ? "type something" : ""}
    />
  </div>
</div>
