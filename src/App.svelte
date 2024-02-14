<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  onMount(async () => {
    await invoke("login");
  });
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>
</main>
