<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import Header from "./lib/Header.svelte";
  import Tables from "./lib/Tables.svelte";
  import Rows from "./lib/Rows.svelte";
  import "./app.css";
  import type { Services } from "./types";

  const services: Services = {
    table: "http://localhost:8080",
    auth: "http://localhost:8081",
    row: "http://localhost:8082",
  };
</script>

<main>
  <Header {services} />
  <Router>
    <div class="container mx-auto pt-4">
      <Route path="/" component={Tables} {services} />
      <Route path="/rows" component={Rows} {services} />
    </div>
  </Router>
</main>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
