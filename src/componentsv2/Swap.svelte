<script>
  // @ts-nocheck

  export let value = "Macro";
  export let onChange;
  let val = ["Action", "Macro", "CustomScript"];
  let idx = 0;
</script>

<label class="swap font-bold">
  <button
    tabindex="-1"
    type="checkbox"
    on:click={(e) => {
      idx = (idx + 1) % 3;
      value = val[idx];
      if (onChange) onChange(value);
    }}
  >
    {value}
  </button>
</label>
