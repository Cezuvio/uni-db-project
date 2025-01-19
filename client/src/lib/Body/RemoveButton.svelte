<script lang="ts">
  import type { Table } from "../../types";

  export let tables: Table[];
  export let serverAddress: string;
  export let index: number;

  let table = tables[index];

  let popup = false;
  let tableToRemove: string | null = null;

  function toggleRemovePopup(name: string | null = null): void {
    popup = !popup;
    tableToRemove = name;
  }

  async function remove(name: string | null): Promise<void> {
    if (name == null) return;

    const response = await fetch(serverAddress + "/tables/" + name, {
      method: "DELETE",
    });

    if (response.ok) tables = tables.filter((table) => table.name != name);
    toggleRemovePopup();
  }
</script>

<div class="ml-auto">
  <button
    class="rounded-lg bg-red-200 p-2"
    on:click={() => toggleRemovePopup(table.name)}>Delete</button
  >
  {#if popup && tableToRemove}
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
          <h2 class="text-lg font-bold">Confirm Deletion</h2>
          <button
            class="text-gray-400 hover:text-white"
            on:click={() => toggleRemovePopup()}
          >
            ×
          </button>
        </div>

        <p class="text-gray-400 mb-4">
          Are you sure you want to delete the table "{tableToRemove}"?
        </p>

        <div class="flex gap-2">
          <button
            class="w-full py-2 bg-red-600 rounded-lg hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
            on:click={() => remove(tableToRemove)}
          >
            Delete
          </button>
          <button
            class="w-full py-2 bg-gray-600 rounded-lg hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500"
            on:click={() => toggleRemovePopup()}
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
