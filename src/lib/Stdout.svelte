<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  interface payload {
    std: string;
  }

  export let goBack: () => void;

  let scrollbox: HTMLDivElement;
  let std: string[] = [];
  let flashEnded: boolean = false;

  onMount(() => {
    listen("flashing_progress", (event) => {
      let payload = event.payload as payload;

      std = [...std, payload.std];
      console.log(payload.std);

      // Scroll manually the div
      setTimeout(() => (scrollbox.scrollTop = scrollbox.scrollHeight), 100);

      // Show a button after last callback
      if (payload.std.includes("Hard resetting")) {
        setTimeout(() => {
          flashEnded = true;

          // Force div to scroll, dirty but useful
          setTimeout(() => (scrollbox.scrollTop = scrollbox.scrollHeight), 100);
        }, 2000);
      }
    });

    // Show button to go back after 15s, this app isn't handle errors so is better than nothing
    setTimeout(() => {
      flashEnded = true;
    }, 15000);
  });
</script>

<div class="mb-5 fixed top-0 left-0 h-full w-full">
  <div
    bind:this={scrollbox}
    class="flex flex-col p-2 h-full w-full overflow-auto bg-slate-500 bg-opacity-80"
  >
    {#each std as line}
      <span>{line}</span>
    {/each}

    <br />

    {#if flashEnded}
      <div>
        <button
          class="mt-4 items-center justify-center p-0.5 overflow-hidden text-sm font-medium rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white text-white focus:ring-4 focus:outline-none focus:ring-cyan-800 disabled:bg-gradient-to-br disabled:from-gray-500 disabled:to-gray-600 disabled:text-gray-500 mx-auto h-[44px]"
          on:click={() => goBack()}
        >
          <span
            class="relative px-8 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0"
          >
            OK
          </span>
        </button>
      </div>
    {/if}
  </div>
</div>
