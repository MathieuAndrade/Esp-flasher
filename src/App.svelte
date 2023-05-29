<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  import Stdout from "./lib/Stdout.svelte";

  let ports: string[] = [];
  let selectedPort: string = "";
  let flashInProgress: boolean = false;

  onMount(async () => {
    ports = await invoke("list_ports");
    selectedPort = ports[0];
  });

  async function flashFirmware() {
    flashInProgress = true;
    setTimeout(async () => {
      await invoke("flash_firmware", { port: selectedPort });
    }, 100);
  }

  async function flashImage() {
    flashInProgress = true;
    setTimeout(async () => {
      await invoke("flash_image", { port: selectedPort });
    }, 100);
  }
</script>

<main
  class="w-full h-screen pt-5 flex flex-col m-auto items-center justify-around"
>
  <h1 class="text-center text-white text-3xl mb-11">Esp Flasher</h1>

  <div class="flex flex-row relative items-center justify-center w-[80%]">
    <label
      for="com-port"
      class="blocktext-sm font-medium dark:text-gray-400 mr-4"
    >
      Selectionnez le port com
    </label>

    <select
      id="com-port"
      class="border cursor-pointer text-sm rounded-lg p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500 w-[40%]"
      bind:value={selectedPort}
    >
      {#each ports as port}
        <option value={port}>{port}</option>
      {/each}
    </select>
  </div>
  <div class="flex flex-row relative items-center mt-5">
    <button
      class="mt-4 relative inline-flex items-center justify-center p-0.5 mb-2 mr-2 overflow-hidden text-sm font-medium rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white text-white focus:ring-4 focus:outline-none focus:ring-cyan-800 disabled:bg-gradient-to-br disabled:from-gray-500 disabled:to-gray-600 disabled:text-gray-500"
      on:click={flashFirmware}
    >
      <span
        class="relative px-8 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0"
      >
        Flasher le frimware
      </span>
    </button>

    <button
      class="mt-4 relative inline-flex items-center justify-center p-0.5 mb-2 mr-2 overflow-hidden text-sm font-medium rounded-lg group bg-gradient-to-br from-cyan-500 to-blue-500 group-hover:from-cyan-500 group-hover:to-blue-500 hover:text-white text-white focus:ring-4 focus:outline-none focus:ring-cyan-800 disabled:bg-gradient-to-br disabled:from-gray-500 disabled:to-gray-600 disabled:text-gray-500"
      on:click={flashImage}
    >
      <span
        class="relative px-8 py-2.5 transition-all ease-in duration-75 bg-white dark:bg-gray-900 rounded-md group-hover:bg-opacity-0"
      >
        Flasher l'image
      </span>
    </button>
  </div>

  {#if flashInProgress}
    <Stdout goBack={() => (flashInProgress = false)} />
  {/if}
</main>
