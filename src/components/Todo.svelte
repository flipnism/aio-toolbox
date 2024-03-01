<script lang="ts">
  import IconButton from "./IconButton.svelte";
  import type { TodoItem, TodoList, UrlPattern } from "../global";
  import { slide } from "svelte/transition";
  import { onMount } from "svelte";
  import todo from "../lib/todohelper";
  import { addToast } from "../lib/toast";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();

  export let todolist: TodoList;
  export let pattern: UrlPattern;
  let show_buttons = false;
  let text_content: TodoItem;
  onMount(() => {
    text_content = todo.hasUrl(todolist.text, pattern);
  });

  async function doCheck(e: any) {
    const result = await todo.todoIsCheck(todolist._id, e.target.checked);
    dispatch("updateTodo");
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  on:mouseleave={() => (show_buttons = false)}
  on:mouseenter={() => (show_buttons = true)}
  class="flex cursor-pointer relative {todolist.checked
    ? ''
    : ''} flex-row odd:bg-base-100 rounded-lg even:bg-base-200"
>
  {#if show_buttons}
    <div
      transition:slide={{ axis: "x", duration: 100 }}
      class="z-10 absolute backdrop-blur-md h-full top-0 right-0 rounded-lg bg-base-300/50 drop-shadow-lg flex p-1 px-4 flex-row gap-4 justify-center items-center w-1/7"
    >
      <IconButton
        on:click={async () => {
          const result = await todo.todoIsDone(todolist._id);
          dispatch("updateTodo");
        }}
        icon="close"
        class="btn-circle btn-xs btn-outline btn-warning"
      />
      <input
        on:change={doCheck}
        type="checkbox"
        bind:checked={todolist.checked}
        class="toggle toggle-xs toggle-warning"
      />
    </div>
  {/if}
  <div class="flex grow px-2 py-2">
    <div
      class="select-none text-sm flex flex-col transition-all {todolist.checked
        ? 'opacity-20 italic line-through line-clamp-1'
        : 'line-clamp-2'}"
    >
      {#if text_content}
        <span
          on:click={() => {
            const content = text_content.text
              .replaceAll("'", "\\'")
              .replaceAll('"', '\\"');
            navigator.clipboard.writeText(content).then((reason) => {
              dispatch("copy", content);
              addToast({
                title: "Save to clipboard : ",
                message: text_content.text.slice(0, 50) + "...",
                type: "error",
                dismissible: false,
                timeout: 3000,
              });
            });
          }}
          class="todotext"
        >
          {text_content.text}
        </span>
        <div class="flex flex-col">
          {#each text_content.links as link, i}
            <a class="text-error pl-2 todolink" target="_blank" href={link}>
              link
            </a>
          {/each}
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .todotext {
    font-size: 10px;
    line-height: 100% !important;
  }
  .todolink {
    font-size: 10px;
  }
</style>
