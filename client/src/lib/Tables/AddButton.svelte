<script lang="ts">
  import type { Table, Services, Row } from "../../types";
  import RowsForm from "./AddButton/RowsForm.svelte";

  export let services: Services;
  export let tables: Table[];
  let name: string | null = null;
  let popup = false;
  let rows: Row[] = [
    {
      name: "",
      column_type: "TEXT",
      is_nullable: false,
      default_value: "",
      is_unique: false,
    },
  ];

  function toggleAddPopup(): void {
    popup = !popup;
    if (popup === false) {
      name = null;
      rows = [
        {
          name: "",
          column_type: "TEXT",
          is_nullable: false,
          default_value: "",
          is_unique: false,
        },
      ];
    }
  }

  function addRowField(): void {
    rows = [
      ...rows,
      {
        name: "",
        column_type: "TEXT",
        is_nullable: false,
        default_value: "",
        is_unique: false,
      },
    ];
  }

  async function addTable(name: string | null, rows: any[]): Promise<void> {
    if (name == null || rows.some((row) => row.name === "")) return;
    const response = await fetch(services.table + "/table", {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: `name=${encodeURIComponent(name)}`,
    });
    if (response.ok) {
      const table: Table = {
        name: name,
      };
      tables = [...tables, table];
    }
  }
</script>

<div class="ml-auto">
  <button
    class="outline outline-2 p-2 outline-white rounded-lg"
    on:click={toggleAddPopup}>Add New Table</button
  >
  {#if popup}
    <div
      class="flex items-center w-full h-full bg-black absolute top-0 left-0"
      style="background-color: rgba(0, 0, 0, 0.6);"
    >
      <div
        class="border-2 border-white rounded-lg bg-gray-800 w-full max-w-4xl mx-auto p-4 text-white"
      >
        <div
          class="flex justify-between items-center border-b border-gray-700 pb-2 mb-4"
        >
          <h2 class="text-lg font-bold">Add New Table</h2>
          <button
            class="text-gray-400 hover:text-white"
            on:click={toggleAddPopup}
          >
            ×
          </button>
        </div>
        <p class="text-gray-400 mb-4">
          Enter the name for the new table and define columns with their
          properties.
        </p>
        <div class="flex items-center gap-2 mb-4">
          <h3 class="text-sm font-medium w-32">Table Name:</h3>
          <input
            bind:value={name}
            type="text"
            class="w-64 px-3 py-2 rounded-lg bg-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div class="mb-4">
          <h3 class="text-sm font-medium mb-2">Columns:</h3>
          <div
            class="grid grid-cols-12 gap-2 mb-2 text-sm font-medium text-gray-400"
          >
            <div class="col-span-3">Column Name</div>
            <div class="col-span-2">Type</div>
            <div class="col-span-4">Default Value</div>
            <div class="col-span-1">Null</div>
            <div class="col-span-1">Unique</div>
            <div class="col-span-1"></div>
          </div>
          <RowsForm bind:rows />
          <button
            class="mt-2 px-3 py-1 bg-green-600 rounded-lg hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500"
            on:click={addRowField}
          >
            + Add Column
          </button>
        </div>
        <button
          class="w-full py-2 bg-blue-600 rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          on:click={() => {
            addTable(name, rows);
            toggleAddPopup();
          }}
        >
          Add Table
        </button>
      </div>
    </div>
  {/if}
</div>
