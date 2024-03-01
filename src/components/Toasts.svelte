<script>
  import { toasts } from "../lib/toast";
  import Toast from "../components/Toast.svelte";
</script>

{#if $toasts}
  {#each $toasts as toast}
    <Toast>
      <span class="font-bold">{toast.title}</span>
      <span class="italic">{toast.message}</span>
    </Toast>
  {/each}
{/if}
