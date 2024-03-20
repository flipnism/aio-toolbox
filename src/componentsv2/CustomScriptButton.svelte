<script>
  import CsButton from "../components/CSButton.svelte";
  /**
   * name
   * desc
   * icon_path
   * executable
   * @type {{ name: any; icon_path: any; } | null}
   */
  export let obj = null;
  export let key = "F24";
</script>

<div class="flex flex-col relative">
  {#if obj}
    <div class="text-[.5rem] font-black">{obj.name}</div>
    <div class="flex justify-between relative w-full">
      <div class="text-[.7rem] font-black">{key}</div>
    </div>

    <CsButton path={obj.icon_path} />
  {/if}
</div>
