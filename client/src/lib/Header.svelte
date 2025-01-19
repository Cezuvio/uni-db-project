<script lang="ts">
  import LoginButton from "./Header/LoginButton.svelte";

  export let serverAddress: string;
</script>

<header class="bg-gray-800 p-4">
  <div class="container mx-auto flex items-center h-full">
    <h1 class="text-2xl font-bold">Admin Panel</h1>
    <LoginButton {serverAddress} />
  </div>
</header>
