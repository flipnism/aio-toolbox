<script lang="ts">
  import GlobalSettings from "./GlobalSettings.svelte";
  import Hotkeys from "./Hotkeys.svelte";
  import Navigation from "./lib/Navigation.svelte";
  import "./styles.css";
  let page = 0;
</script>

<div class="flex flex-col">
  <Navigation bind:page_state={page}>
    {#if page == 0}
      <Hotkeys />
    {:else if page === 1}
      <GlobalSettings />
    {/if}
  </Navigation>
</div>
