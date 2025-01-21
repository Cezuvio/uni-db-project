<script lang="ts">
  import { warnings } from "./stores/store";
  import { Router, Route } from "svelte-routing";
  import Header from "./lib/Header.svelte";
  import Tables from "./lib/Tables.svelte";
  import Rows from "./lib/Rows.svelte";
  import "./app.css";

  const removeWarning = (index: number) => {
    warnings.update((warnings) => {
      return warnings.filter((_, i) => i !== index);
    });
  };
</script>

<main>
  {#each $warnings as warning, i}
    <div
      class="fixed top-10 left-1/2 transform -translate-x-1/2 w-auto max-w-md bg-red-500 text-white p-4 rounded shadow-lg z-50 flex items-center justify-between"
    >
      <span>{warning}</span>
      <button
        class="pl-2 text-white font-bold"
        on:click={() => removeWarning(i)}
      >
        &times;
      </button>
    </div>
  {/each}

  <Header />
  <Router>
    <div class="container mx-auto pt-4">
      <Route path="/" component={Tables} />
      <Route path="/rows" component={Rows} />
    </div>
  </Router>
</main>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
