<script lang="ts">
  import { show } from "@tauri-apps/api/app";
  import ArrayInputText from "../components/ArrayInputText.svelte";
  import InputSwitch from "../components/InputSwitch.svelte";
  import InputText from "../components/InputText.svelte";
  import type { GlobalSetting } from "../global";
  import { createEventDispatcher } from "svelte";
  import InputBoolean from "../components/InputBoolean.svelte";

  const dispatch = createEventDispatcher();

  export let show_modal = false;

  export let modal_data: GlobalSetting = {
    name: "something",
    type: "string",
    values: [],
  };
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<dialog tabindex="0" open={show_modal} id="my_modal_2" class="modal">
  <div class="modal-box flex flex-col gap-2">
    <h3 class="font-bold text-lg">Setting Configuration</h3>
    <InputText title="Name" bind:value={modal_data.name} />
    <InputSwitch
      title="Type"
      options={["string", "array", "number", "bool"]}
      bind:value={modal_data.type}
    />
    {#if modal_data.type === "string" || modal_data.type === "number"}
      <InputText
        title="Value"
        type={modal_data.type}
        bind:value={modal_data.value}
      />
    {:else if modal_data.type === "bool"}
      <InputBoolean title="Value" bind:value={modal_data.value} />
    {:else}
      <ArrayInputText title="Value" bind:values={modal_data.values} />
    {/if}
    <div class="modal-action">
      <form method="dialog">
        <button
          on:click={() => (show_modal = false)}
          class="btn btn-sm btn-error"
        >
          Cancel
        </button>
        <button
          on:click={() => {
            show_modal = false;
            dispatch("data", modal_data);
          }}
          class="btn btn-sm"
        >
          Add
        </button>
      </form>
    </div>
  </div>
</dialog>
