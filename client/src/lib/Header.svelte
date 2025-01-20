<script lang="ts">
  import LoginButton from "./Header/LoginButton.svelte";
  import type { Services } from "../types.ts";

  export let services: Services;
</script>

<header class="bg-gray-800 p-4">
  <div class="container mx-auto flex items-center h-full">
    <h1 class="text-2xl font-bold">Admin Panel</h1>
    <LoginButton {services} />
  </div>
</header>
