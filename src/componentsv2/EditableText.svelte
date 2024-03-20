<script>
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
  export let value = "type something";
  let editmode = false;
  let inputref;
</script>

<div class="flex w-full items-center text-[0.8rem]">
  <input
    bind:this={inputref}
    on:focusout={() => {
      editmode = false;
    }}
    type="text"
    class="input input-ghost input-xs w-full"
    bind:value
  />
</div>
