<script lang="ts">
  import type { Row } from "../../../types";

  export let rows: Row[];

  function removeRowField(index: number): void {
    rows = rows.filter((_, i) => i !== index);
  }
</script>

{#each rows as row, index}
  <div class="grid grid-cols-12 gap-2 mb-2">
    <input
      bind:value={row.name}
      type="text"
      placeholder="Column name"
      class="col-span-3 px-3 py-2 rounded-lg bg-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
    <select
      bind:value={row.column_type}
      class="col-span-2 px-3 py-2 rounded-lg bg-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    >
      <option value="TEXT">TEXT</option>
      <option value="INTEGER">INTEGER</option>
      <option value="FLOAT">FLOAT</option>
      <option value="BOOLEAN">BOOLEAN</option>
      <option value="DATE">DATE</option>
      <option value="TIMESTAMP">TIMESTAMP</option>
    </select>
    <input
      bind:value={row.default_value}
      type="text"
      placeholder="Default value"
      class="col-span-4 px-3 py-2 rounded-lg bg-gray-700 text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
    />
    <div class="col-span-1 flex items-center justify-center">
      <input
        bind:checked={row.is_nullable}
        type="checkbox"
        class="w-4 h-4 rounded focus:ring-2 focus:ring-blue-500"
      />
    </div>
    <div class="col-span-1 flex items-center justify-center">
      <input
        bind:checked={row.is_unique}
        type="checkbox"
        class="w-4 h-4 rounded focus:ring-2 focus:ring-blue-500"
      />
    </div>
    {#if rows.length > 1}
      <button
        class="col-span-1 px-2 py-1 bg-red-600 rounded-lg hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500"
        on:click={() => removeRowField(index)}
      >
        ×
      </button>
    {/if}
  </div>
{/each}
