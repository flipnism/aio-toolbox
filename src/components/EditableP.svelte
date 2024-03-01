<script lang="ts">
  export let value = "";
  let ref: HTMLInputElement;
  let edit_mode = false;
</script>

<div class="flex {$$props.class}">
  <button
    class="select-none {edit_mode ? 'hidden' : ''}"
    on:click={() => {
      edit_mode = true;
      setTimeout(() => {
        ref.focus();
      }, 100);
    }}
  >
    {value}
  </button>
  <input
    on:focusout={() => {
      edit_mode = false;
    }}
    bind:this={ref}
    type="text"
    class="input input-sm {edit_mode ? '' : 'hidden'}"
    bind:value
  />
</div>
