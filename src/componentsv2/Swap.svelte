<script>
  // @ts-nocheck

  export let value = "Macro";
  export let onChange;
  let val = ["Action", "Macro", "CustomScript","AppFunction"];
  let idx = 0;
</script>

<label class="swap font-bold">
  <button
    class="text-[.7rem]"
    tabindex="-1"
    type="checkbox"
    on:click={(e) => {
      idx = (idx + 1) % 4;
      value = val[idx];
      if (onChange) onChange(value);
    }}
  >
    {value}
  </button>
</label>
