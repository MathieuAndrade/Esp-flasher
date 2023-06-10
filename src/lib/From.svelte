<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  export let step = 0;

  let ports: string[] = [];

  let selectedPort: string = "default";
  let selectedFilename = "";
  let selectedFilePath = "";

  const openDialog = () => invoke("open_file_dialog");

  onMount(async () => {
    ports = await invoke("list_ports");

    listen("tauri://file-drop", (e) => {
      selectedFilePath = e.payload[0];
      selectedFilename = selectedFilePath.replace(/^.*[\\\/]/, "");
      step = 1;
    });

    listen("file_selected", (e) => {
      selectedFilePath = e.payload as string;
      selectedFilename = selectedFilePath.replace(/^.*[\\\/]/, "");
      step = 1;
    });

    // Reset the form when flashing is finished
    listen("flash_finished", () => {
      step = 0;
      selectedPort = "default";
      selectedFilename = "";
      selectedFilePath = "";
    });
  });

  function onPortSelected() {
    if (selectedPort !== "default") {
      step = 2;
    }
  }

  async function flashFirmware() {
    invoke("flash_firmware", {
      port: selectedPort,
      file: selectedFilePath,
    });
  }

  async function flashImage() {
    invoke("flash_image", {
      port: selectedPort,
      file: selectedFilePath,
    });
  }
</script>

<div class="w-full flex justify-between items-end mt-5">
  <div>
    <div class="flex items-center justify-center w-full">
      <label
        for="file-uploader"
        class="min-w-[125px] text-white bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-blue-800 shadow-lg shadow-blue-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-2 mb-2 cursor-pointer"
      >
        {#if selectedFilename === ""}
          Select image
        {:else}
          {selectedFilename}
        {/if}
        <button id="file-uploader" class="hidden" on:click={openDialog} />
      </label>
    </div>
  </div>
  <div class="mb-2 text-center">
    <select
      id="com-port"
      class="border text-sm rounded-lg block w-full p-2.5 bg-gray-700 border-gray-600 placeholder-gray-400 text-white focus:ring-blue-500 focus:border-blue-500 cursor-pointer disabled:cursor-not-allowed"
      bind:value={selectedPort}
      disabled={selectedFilename === ""}
      on:change={onPortSelected}
    >
      <option value="default">Select port</option>
      {#each ports as port}
        <option value={port}>{port}</option>
      {/each}
    </select>
  </div>
  <div>
    <button
      id="flash"
      type="button"
      class="min-w-[125px] text-white bg-gradient-to-r from-blue-500 via-blue-600 to-blue-700 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-blue-800 shadow-lg shadow-blue-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center mr-2 mb-2 disabled:cursor-not-allowed disabled:from-blue-800 disabled:from-to-800 disabled:hover:bg-gradient-to-r disabled:shadow-none"
      disabled={selectedFilename === "" || selectedPort === "default"}
      on:click={flashFirmware}
    >
      Flash
    </button>
  </div>
</div>
