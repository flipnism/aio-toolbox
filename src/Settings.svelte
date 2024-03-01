<script lang="ts">
  import GlobalSettings from "./GlobalSettings.svelte";
  import Hotkeys from "./Hotkeys.svelte";
  import TodoLust from "./TodoLust.svelte";
  import Navigation from "./lib/Navigation.svelte";
  import "./styles.css";
  let page = 2;
</script>

<div class="flex flex-col h-screen setting">
  <Navigation bind:page_state={page}>
    {#if page == 0}
      <Hotkeys />
    {:else if page === 1}
      <GlobalSettings />
    {:else if page === 2}
      <TodoLust />
    {/if}
  </Navigation>
</div>

<style>
</style>
