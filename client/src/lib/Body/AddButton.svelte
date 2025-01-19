<script lang="ts">
  import type { Table } from "../../types";

  export let serverAddress: string;
  export let tables: Table[];

  let name: string | null = null;
  let popup = false;

  function toggleAddPopup(): void {
    popup = !popup;
    if (popup === false) name = null;
  }

  async function addTable(name: string | null): Promise<void> {
    if (name == null) return;

    const response = await fetch(serverAddress + "/table", {
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
        class="border-2 border-white rounded-lg bg-gray-800 w-full max-w-md mx-auto p-4 text-white"
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
          Enter the name for the new table you want to add.
        </p>

        <div class="flex items-center gap-2 mb-4">
          <h3 class="text-sm font-medium w-1/4">Name:</h3>
          <input
            bind:value={name}
            type="text"
            class="w-3/4 px-3 py-2 rounded-lg bg-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <button
          class="w-full py-2 bg-blue-600 rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          on:click={() => {
            addTable(name);
            toggleAddPopup();
          }}
        >
          Add Table
        </button>
      </div>
    </div>
  {/if}
</div>
