<script lang="ts">
  import Header from "./lib/Header.svelte";
  import Body from "./lib/Body.svelte";
  import "./app.css";

  const serverAddress = "http://localhost:8080";
</script>

<main>
  <Header {serverAddress} />
  <Body {serverAddress} />
</main>

<style lang="postcss">
  :global(html) {
    background-color: theme(colors.gray.100);
  }
</style>
