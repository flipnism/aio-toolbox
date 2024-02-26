<script lang="ts">
  import { fade } from "svelte/transition";
  import IconButton from "../components/IconButton.svelte";
  import { elasticInOut, quadInOut } from "svelte/easing";
  let key_macro: any = [];

  let current_key_down = "";
  export let show_macropanel = true;
  let is_macro = true;
  let macro_record_enable = false;
  let current_macro_title = "Untitled";
  function doRefresh() {
    key_macro = [];
  }
  function doSave() {
    console.log(key_macro);
  }
  function doClose() {
    show_macropanel = false;
    key_macro = [];
    current_key_down = "";
    macro_record_enable = false;
  }
</script>

<svelte:document
  on:keydown={(e) => {
    if (!is_macro || !macro_record_enable) return;
    const key = {
      key: e.key,
      down: true,
    };

    if (current_key_down != e.key) {
      key_macro = [...key_macro, key];
    }

    current_key_down = e.key;
  }}
  on:keyup={(e) => {
    if (!is_macro || !macro_record_enable) return;

    const key = {
      key: e.key,
      down: false,
    };

    key_macro = [...key_macro, key];

    current_key_down = "";
  }}
/>
{#if show_macropanel}
  <div
    transition:fade={{ duration: 250, easing: quadInOut }}
    class={`select-none ` + $$props.class || ""}
  >
    <div class="flex flex-row gap-2 items-center justify-between">
      <div class="flex gap-2">
        <p class="text-sm">Macro title</p>
        <input
          disabled={macro_record_enable}
          bind:value={current_macro_title}
          type="text"
          class="input input-bordered input-xs max-w-lg"
          placeholder="macro name"
        />
      </div>
      <div class="flex items-center gap-2">
        <label class="label cursor-pointer gap-2 items-center">
          <span class="label-text">rec</span>
          <input
            bind:checked={macro_record_enable}
            type="checkbox"
            class="toggle toggle-xs toggle-error"
          />
        </label>
        <IconButton
          on:click={doRefresh}
          icon="refresh"
          class="btn-outline btn-circle btn-neutral"
        />
        <IconButton
          icon="save"
          on:click={doSave}
          class="btn-outline btn-circle btn-success"
        />
        <IconButton
          on:click={doClose}
          icon="close"
          class="btn-outline btn-circle btn-error"
        />
      </div>
    </div>
    <div class="macro_content">
      <div class="flex flex-row gap-2 flex-wrap p-4">
        {#each key_macro as macro, i}
          <div
            transition:fade
            class="flex flex-col items-center justify-center select-none"
          >
            <div>
              {#if !macro.down}
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 16 16"
                  fill="currentColor"
                  class="w-4 h-4"
                >
                  <path
                    fill-rule="evenodd"
                    d="M11.78 9.78a.75.75 0 0 1-1.06 0L8 7.06 5.28 9.78a.75.75 0 0 1-1.06-1.06l3.25-3.25a.75.75 0 0 1 1.06 0l3.25 3.25a.75.75 0 0 1 0 1.06Z"
                    clip-rule="evenodd"
                  />
                </svg>
              {/if}
            </div>
            <kbd class="kbd kbd-sm">
              {macro.key}
            </kbd>
            <div>
              {#if macro.down}
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 16 16"
                  fill="currentColor"
                  class="w-4 h-4"
                >
                  <path
                    fill-rule="evenodd"
                    d="M4.22 6.22a.75.75 0 0 1 1.06 0L8 8.94l2.72-2.72a.75.75 0 1 1 1.06 1.06l-3.25 3.25a.75.75 0 0 1-1.06 0L4.22 7.28a.75.75 0 0 1 0-1.06Z"
                    clip-rule="evenodd"
                  />
                </svg>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
{/if}
