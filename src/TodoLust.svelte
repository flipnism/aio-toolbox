<script lang="ts">
  import { onMount } from "svelte";
  import todo, { urlPattern } from "./lib/todohelper";
  import type { TodoList, UrlPattern } from "./global";
  import Todo from "./components/Todo.svelte";
  import IconButton from "./components/IconButton.svelte";
  import { fade, slide } from "svelte/transition";
  import { cubicInOut, quartInOut } from "svelte/easing";
  import Toasts from "./components/Toasts.svelte";
  import { addToast } from "./lib/toast";
  import { invoke } from "@tauri-apps/api";
  import { writable } from "svelte/store";
  import { listen } from "@tauri-apps/api/event";
  let pattern = urlPattern();
  let todolist: TodoList[];

  async function updateTodo() {
    const newlist = await todo.fetchList();
    todolist = newlist;
  }
  listen("socket_message", async (message) => {
    const data: any = message.payload;
    switch (data.type) {
      case "chat":
      case "chat_update":
        todolist = await todo.fetchList();
        break;
    }
  });

  onMount(async () => {
    todolist = await todo.fetchList();
  });
  let show_generate_dialog = false;
  let todo_text = "";
</script>

<svelte:document />
<div
  in:fade={{ delay: 100 }}
  out:fade={{ duration: 100 }}
  class="flex flex-col gap-4 overflow-y-auto p-4 h-full"
>
  {#if show_generate_dialog}
    <div
      transition:fade={{ duration: 200, easing: cubicInOut }}
      class="flex flex-col absolute z-30 bg-base-20 top-0 left-0 w-full h-full"
    >
      <button
        on:click={() => (show_generate_dialog = false)}
        class="flex-1 bg-base-300/50 w-full backdrop-blur-sm"
      ></button>
      <div
        class="flex flex-col gap-2 bg-base-300 w-full p-4 drop-shadow-[0_-10px_15px_rgba(0,0,0,.5)]"
      >
        <textarea
          bind:value={todo_text}
          class="textarea textarea-bordered min-h-36"
        />
        <button
          on:click={() => {
            todo.todoAdd(todo_text).then((result) => {
              updateTodo();
              show_generate_dialog = false;
            });
          }}
          class="btn btn-neutral btn-xs self-end font-black uppercase"
        >
          Generate
        </button>
      </div>
    </div>
  {/if}

  <div class="flex flex-col mb-12">
    {#if todolist && todolist.length > 0}
      {#each todolist as td (td._id)}
        <Todo
          todolist={td}
          {pattern}
          on:updateTodo={updateTodo}
          on:copy={(result) => {
            invoke("send_socket_message", {
              message: {
                fromserver: false,
                type: "todo_clipboard",
                data: result.detail,
              },
            })
              .then((result) => console.log(result))
              .catch((e) => console.log(e));
            // todo.sendText(socket, result.detail);
          }}
        />
      {/each}
    {/if}
  </div>

  <footer
    class="z-20 select-none px-8 py-4 w-full flex justify-end right-0 fixed bottom-0 gap-2"
  >
    <IconButton
      on:click={updateTodo}
      class="btn-warning  btn-circle"
      icon="refresh"
    />
    <IconButton
      on:click={() => {
        todo.deleteList().then((result) => {
          updateTodo();
        });
      }}
      class="btn-warning  btn-circle"
      icon="deleteall"
    />
    <IconButton
      on:click={async () => {
        const done = todolist.filter((todo) => todo.checked).map((d) => d._id);
        for await (const d of done) {
          await todo.todoIsDone(d);
        }
        updateTodo();
      }}
      tooltip_text="delete all checked"
      icon="delete"
      class="btn-warning btn-circle"
    />
    <IconButton
      tooltip_text="add todo"
      on:click={() => {
        show_generate_dialog = true;
      }}
      icon="add"
      class="btn-warning  btn-circle"
    />
  </footer>
</div>
<Toasts />

<style>
  * {
    font-family: "Bahnschrift";
  }
</style>
