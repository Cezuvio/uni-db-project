<script lang="ts">
  import { onMount } from "svelte";
  import type { Table } from "../types";
  import AddButton from "./Body/AddButton.svelte";
  import RemoveButton from "./Body/RemoveButton.svelte";

  export let serverAddress: string;

  let tables: Table[] = [];

  onMount(async () => {
    tables = await fetchTables();
  });

  async function fetchTables(): Promise<Table[]> {
    const response = await fetch(serverAddress + "/tables");
    const result = await response.json();

    return result.map((item: { Table: Table }) => item.Table);
  }
</script>

<div class="container mx-auto pt-4">
  <div class="flex items-center mb-4 w-full">
    <h1 class="text-2xl font-bold">Database Administration Panel</h1>
    <AddButton bind:tables {serverAddress} />
  </div>

  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each tables as table, index}
      <div
        class="p-6 bg-gray-800 rounded-lg border border-gray-700 shadow-md hover:bg-gray-700 flex items-center"
      >
        <h2 class="text-xl font-bold">{table.name}</h2>
        <RemoveButton bind:tables {serverAddress} {index} />
      </div>
    {/each}
  </div>
</div>
