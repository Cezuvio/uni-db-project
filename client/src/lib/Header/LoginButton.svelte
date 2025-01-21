<script lang="ts">
  import { services } from "../../stores/store";

  let popup = false;
  let username = "";
  let password = "";

  function togglePopup(): void {
    popup = !popup;
  }

  async function sendLoginRequest(
    name: string,
    password: string,
  ): Promise<void> {
    try {
      const response = await fetch($services.auth + "/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/x-www-form-urlencoded",
        },
        body: `name=${encodeURIComponent(name)}&password=${encodeURIComponent(password)}`,
      });

      console.log(await response.json());
    } catch (error) {}
  }
</script>

<div class="ml-auto">
  <button
    on:click={togglePopup}
    class="rounded-lg bg-red-200 p-2 text-gray-800 font-semibold hover:bg-red-300 transition duration-200"
  >
    Admin Login
  </button>

  {#if popup}
    <div
      class="flex items-center justify-center w-full h-full bg-black absolute top-0 left-0"
      style="background-color: rgba(0, 0, 0, 0.6);"
    >
      <div
        class="border-2 border-white rounded-lg bg-gray-800 w-full max-w-md mx-auto p-6 text-white"
      >
        <div
          class="flex justify-between items-center border-b border-gray-700 pb-2 mb-4"
        >
          <h2 class="text-lg font-bold">Admin Login</h2>
          <button
            class="text-gray-400 hover:text-white"
            on:click={() => togglePopup()}
          >
            ×
          </button>
        </div>

        <form class="space-y-4">
          <div>
            <label for="username" class="block text-gray-400">Username</label>
            <input
              bind:value={username}
              id="username"
              type="text"
              class="w-full p-2 mt-2 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Enter your username"
            />
          </div>

          <div>
            <label for="password" class="block text-gray-400">Password</label>
            <input
              bind:value={password}
              id="password"
              type="password"
              class="w-full p-2 mt-2 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              placeholder="Enter your password"
            />
          </div>

          <div class="flex gap-4">
            <button
              class="w-full py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
              on:click={() => {
                sendLoginRequest(username, password);
                togglePopup();
              }}
            >
              Login
            </button>
            <button
              type="button"
              class="w-full py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500"
              on:click={() => togglePopup()}
            >
              Cancel
            </button>
          </div>
        </form>
      </div>
    </div>
  {/if}
</div>
