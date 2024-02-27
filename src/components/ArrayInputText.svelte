<script lang="ts">
  import { tags, writable } from "../lib/store";
  import { fade } from "svelte/transition";

  type Item = {
    id: number;
    value: string;
  };
  export let title = "Title here:";
  const local_tags = writable([]);
  let page_tags: Item[] = [];
  local_tags.subscribe((tagitem: Item[]) => (tagitem = page_tags));

  let text = "";
  let idx = 0;
  function addItem(e: any) {
    if (e.key == "Backspace") {
      const data = page_tags[page_tags.length - 1];
      if (text !== "") return;
      removeItem(data.id);
      return;
    }
    if (e.key == "Enter") {
      if (e.target.value.trim() == "") return;

      idx = idx + 1;

      page_tags = [...page_tags, { id: idx, value: e.target.value }];
      console.log(page_tags);

      text = "";
    }
  }

  function removeItem(id: number) {
    page_tags = page_tags.filter((tag) => tag.id != id);
    console.log(page_tags);
  }
</script>

<div
  out:fade={{ duration: 200 }}
  in:fade={{ duration: 200, delay: 200 }}
  class="flex flex-row"
>
  <p class="font-black px-2 w-1/3 self-center">{title}</p>
  <div
    class="flex w-full bg-base-200 rounded-lg flex-row gap-1 flex-wrap items-center p-2"
  >
    {#if page_tags}
      {#each page_tags as tag, i}
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
      tabindex="0"
      bind:value={text}
      on:keydown={addItem}
      type="text"
      class="grow input input-xs input-ghost border-none focus:outline-none"
      placeholder={page_tags.length <= 0 ? "type something" : ""}
    />
  </div>
</div>
