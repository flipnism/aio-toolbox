<script lang="ts">
  import { fade } from "svelte/transition";
  import AppConfig from "./AppConfig.svelte";
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
      <AppConfig />
    {:else if page === 2}
      <TodoLust />
    {/if}
  </Navigation>
</div>

<style>
  * {
    font-family: "Bahnschrift";
  }
</style>
