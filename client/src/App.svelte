<script lang="ts">
  import Header from "./lib/Header.svelte";
  import Body from "./lib/Body.svelte";
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
  <Body {services} />
</main>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
