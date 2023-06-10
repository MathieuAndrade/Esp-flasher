<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  let isFlashing = false;
  let flashProgress = 0;

  onMount(async () => {
    listen("flash_started", () => {
      isFlashing = true;
    });
    listen("flash_progress_update", (e) => {
      let payload = e.payload as { msg: string };
      flashProgress = Number(payload.msg);
    });
    listen("flash_finished", () => {
      isFlashing = false;
    });
  });
</script>

{#if isFlashing}
  <div class="w-full bg-gray-200 rounded-full dark:bg-gray-700 mt-5">
    <div
      class="bg-blue-600 text-xs font-medium text-blue-100 text-center p-0.5 leading-none rounded-full"
      style:width="{flashProgress}%"
    >
      {flashProgress}%
    </div>
  </div>
{/if}
