<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  const circleSize = 30 * 2 * Math.PI;

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
  <div
    class="fixed flex flex-col items-center justify-center w-full h-full bg-gray-200/60 rounded-lg"
  >
    <div class="inline-flex items-center justify-center">
      <!-- Building a Progress Ring: https://css-tricks.com/building-progress-ring-quickly/ -->
      <svg class="w-20 h-20">
        <circle
          class="text-gray-300"
          stroke-width="5"
          stroke="currentColor"
          fill="transparent"
          r="30"
          cx="40"
          cy="40"
        />
        <circle
          class="text-blue-600"
          stroke-width="5"
          stroke-dasharray={circleSize}
          stroke-dashoffset={circleSize - (circleSize / 100) * flashProgress}
          stroke-linecap="round"
          stroke="currentColor"
          fill="transparent"
          r="30"
          cx="40"
          cy="40"
        />
      </svg>
      <span class="absolute text-xl text-blue-700 font-semibold">
        {flashProgress}%
      </span>
    </div>
    <span class="text-md text-blue-700 font-semibold">Flash in progress...</span
    >
  </div>
{/if}
