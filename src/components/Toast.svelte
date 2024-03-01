<script>
  import { onMount } from "svelte";
  import { slide } from "svelte/transition";
</script>

<div
  transition:slide={{ axis: "y", duration: 200 }}
  class="z-50 text-xs w-1/2 absolute bottom-4 bg-warning left-4 text-warning-content p-2 px-8 rounded-lg"
>
  <slot />
</div>
