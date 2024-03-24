<script>
  import { onMount } from "svelte";
  import Dropdown from "./componentsv2/Dropdown.svelte";
  import EditableText from "./componentsv2/EditableText.svelte";
  import IconButton from "./componentsv2/IconButton.svelte";
  import Swap from "./componentsv2/Swap.svelte";
  import { invoke } from "@tauri-apps/api";
  import {
    BaseDirectory,
    readTextFile,
    writeTextFile,
  } from "@tauri-apps/api/fs";
  import MacroRecord from "./componentsv2/MacroRecord.svelte";
  import { fade } from "svelte/transition";
    import { MultiDrag } from "sortablejs";
  /**
   * @type {any[]}
   */
  let items = [];
  /**
   * @type {any[]}
   */
  let stored_items = [];
  let key_items = [
    "F13",
    "F14",
    "F15",
    "F16",
    "F17",
    "F18",
    "F19",
    "F20",
    "F21",
    "F22",
    "F23",
    "F24",
  ];
  /**
   * @type {never[]}
   */
  let action_lists = [];
  /**
   * @type {never[]}
   */
  let script_lists = [];
  /**
   * @type {never[]}
   */
  let customScripts = [];

  let appfunction_lists = [];
  onMount(() => {
    invoke("list_customscripts").then((result) => {
      customScripts = result.map((/** @type {any[]} */ item) => item[0]);
    });
    readTextFile("config.json", { dir: BaseDirectory.AppData }).then(
      (result) => {
        items = JSON.parse(result);
        stored_items = items;
      },
    );

    readTextFile("appfunction_list.json", { dir: BaseDirectory.AppData })
      .then((result) => {
        appfunction_lists = JSON.parse(result);
      })
      .catch((e) => {
        writeTextFile("appfunction_list.json", JSON.stringify([]), {
          dir: BaseDirectory.AppData,
        });
      });

    invoke("script_lists").then((result) => {
      script_lists = result;
    });
    invoke("action_lists")
      .then((result) => {
        action_lists = result;
      })
      .catch((e) => console.log(e));
  });

  let active_data = {
    macro_id: -1,
    show: false,
  };
  /**
   * @param {{ Macro: any[]; } | null} data
   */
  function macros(data) {
    return data != null && data.Macro && data.Macro.length > 0
      ? [
          ...new Set(
            data.Macro.map((/** @type {{ key: any; }} */ e) => {
              return e.key;
            }),
          ),
        ]
          .join(" ")
          .toUpperCase()
      : "Macro";
  }

  function writeConfig() {
    // const data = items.map((it) => {
    //   let newKey;
    //   switch (it.key_mode) {
    //     case "Macro":
    //       newKey = {
    //         Macro: { ...it.key_data },
    //       };

    //       break;
    //     default:
    //       newKey = {
    //         Action: { ...it.key_data },
    //       };
    //       break;
    //   }

    //   return {
    //     ...it,
    //     key_data: newKey,
    //   };
    // });

    writeTextFile("config.json", JSON.stringify(items, null, 2), {
      dir: BaseDirectory.AppData,
    });
  }
  $:console.log(multikey_filter_only);
  let multikey_filter_only = false;
  let filterText = "";
  function doFilter() {
    console.log("filter");
    items = stored_items.filter((item) => {
      if (multikey_filter_only) {
        return item.key_1.includes(filterText);
      } else {
        return (
          item.key_name.includes(filterText) ||
          item.key_desc.includes(filterText)
        );
      }
    });
  }
</script>

<MacroRecord
  bind:macro_id={active_data.macro_id}
  bind:show={active_data.show}
  on:save={(result) => {
    const d = result.detail;
    items[d.id].key_data = { Macro: d.value };
  }}
/>

<div
  in:fade={{ delay: 100 }}
  out:fade={{ duration: 100 }}
  class="flex flex-col h-full overflow-y-auto text-[.5rem]"
>
  <div
    class="flex p-4 gap-5 justify-end fixed bg-base-300/90 h-fit w-full backdrop-blur-sm z-40"
  >
    <label
      class="input input-ghost input-bordered input-sm flex items-center gap-2"
    >
      <input
        bind:value={filterText}
        on:input={doFilter}
        type="text"
        class="grow"
      />
      <input
        bind:checked={multikey_filter_only}
        type="checkbox"
        class="checkbox checkbox-xs"
      />
      <IconButton
        on:click={() => {
          filterText = "";
          doFilter();
        }}
        icon="delete"
        class="btn-xs btn-ghost"
      />
    </label>

    <div class="h-2 flex-1"></div>
    <button on:click={writeConfig} class="btn btn-sm btn-neutral">
      Generate
    </button>
    <IconButton
      icon="add"
      on:click={() => {
        items.push({
          key_name: "some",
          key_desc: "some",
          key_mode: "Action",
          key_multikey: false,
          key_1: "",
          key_2: "",
          key_data: {
            Action: {
              fromserver: false,
              type: "hotkey",
              data: "None",
            },
          },
        });
        items = items;
      }}
    />
  </div>

  <table class="table mt-16 table-zebra table-xs">
    <thead>
      <tr class="bg-base-300 uppercase font-black">
        <th></th>
        <th>Name</th>
        <th>Desc</th>
        <th>Mode</th>
        <th>Multi Key</th>
        <th>Key(s)</th>
        <th>Data</th>
        <th></th>
      </tr>
    </thead>
    <tbody>
      {#each items as item, i}
        <tr class="hover">
          <td>{i + 1}</td>
          <td>
            <EditableText bind:value={item.key_name} />
          </td>
          <td>
            <EditableText bind:value={item.key_desc} />
          </td>
          <td>
            <Swap
              bind:value={item.key_mode}
              onChange={(/** @type {string} */ e) => {
                if (e == "Action") {
                  item.key_data = {
                    Action: {
                      fromserver: false,
                      type: "hotkey",
                      data: "None",
                    },
                  };
                } else if (e == "Macro") {
                  item.key_data = { Macro: [] };
                } else {
                  item.key_data = {
                    CustomScript: customScripts,
                  };
                }
              }}
            />
          </td>
          <td>
            <input
              tabindex="-1"
              bind:checked={item.key_multikey}
              type="checkbox"
              class="toggle toggle-xs toggle-error"
            />
          </td>
          <td class="flex flex-row">
            <Dropdown
              selected={item.key_1 != "" ? item.key_1 : "None"}
              items={key_items}
              on:selected={(e) => {
                item.key_1 = e.detail;
                console.log(e.detail);
              }}
            />
            {#if item.key_multikey}
              <Dropdown
                selected={item.key_2 != "" ? item.key_2 : "None"}
                items={key_items}
                on:selected={(e) => {
                  item.key_2 = e.detail;
                  console.log(e.detail);
                }}
              />
            {/if}
          </td>
          <td>
            {#if item.key_mode == "Action"}
              <Dropdown
                selected={item.key_data ? item.key_data.Action.data : "None"}
                items={action_lists}
                on:selected={(e) => {
                  item.key_data = {
                    Action: {
                      fromserver: false,
                      type: "hotkey",
                      data: e.detail,
                    },
                  };
                }}
              />
            {:else if item.key_mode == "Macro"}
              <button
                on:click={() => {
                  active_data.show = true;
                  active_data.macro_id = i;
                }}
                class="btn btn-xs"
              >
                {macros(item.key_data)}
              </button>
            {:else if item.key_mode == "CustomScript"}
              <Dropdown
                selected={item.key_data
                  ? item.key_data.CustomScript.data
                  : "None"}
                items={script_lists}
                on:selected={(e) => {
                  item.key_data = {
                    CustomScript: { data: e.detail },
                  };
                }}
              />
            {:else}
              <Dropdown
                selected={item.key_data?item.key_data.AppFunction:"None"}
                items={appfunction_lists}
                on:selected={(e) => {
                item.key_data = {
                  AppFunction:e.detail
              }
              }}
              />
            {/if}
          </td>
          <td>
            <IconButton
              on:click={() => {
                items = [...items.filter((it) => it != item)];
              }}
              class="btn-ghost"
              icon="delete"
            />
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<style>
  * {
    font-family: "Bahnschrift";
  }
  *::-webkit-scrollbar {
    width: 5px;
  }

  *::-webkit-scrollbar-track {
    @apply bg-base-100;
  }

  *::-webkit-scrollbar-thumb {
    @apply bg-base-content/10;
  }
  *::-webkit-scrollbar-thumb:hover {
    @apply bg-base-content;
    cursor: grabbing;
  }
</style>
