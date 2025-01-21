<script lang="ts">
  import { services } from "../stores/store";
  import { Link } from "svelte-routing";

  import { onMount } from "svelte";
  import type { Table } from "../types";
  import AddButton from "./Tables/AddButton.svelte";
  import RemoveButton from "./Tables/RemoveButton.svelte";

  let tables: Table[] = [];

  onMount(async () => {
    tables = await fetchTables();
  });

  async function fetchTables(): Promise<Table[]> {
    const response = await fetch($services.table + "/tables");
    const result = await response.json();
    return result.map((item: { Table: Table }) => item.Table);
  }
</script>

<div class="flex items-center mb-4 w-full">
  <h1 class="text-2xl font-bold">Database Administration Panel</h1>
  <AddButton bind:tables />
</div>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
  {#each tables as table, index}
    <div
      class="p-6 bg-gray-800 rounded-lg border border-gray-700 shadow-md hover:bg-gray-700"
    >
      <div class="flex items-center">
        <h2 class="text-xl font-bold">{table.name}</h2>
        {#if table.name !== "admins"}
          <RemoveButton bind:tables {index} />
        {/if}
      </div>
      <Link to="/rows" class="text-blue-500 underline"
        >View and manage users</Link
      >
    </div>
  {/each}
</div>
