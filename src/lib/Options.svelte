<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, Label, Radio, Select } from "flowbite-svelte";
  import { onMount } from "svelte";

  let availablePorts: Array<{ value: string; name: string }> = [];
  let selectedPort: string = "";
  let selectedDeviceType: string = "8266";
  let selectedFilename = "";
  let selectedFilePath = "";
  let flashType = "firmware";

  const openDialog = () => invoke("open_file_dialog");

  onMount(async () => {
    let ports: string[] = await invoke("list_ports");
    availablePorts = ports.map((port) => ({ value: port, name: port }));
    selectedPort = availablePorts[0].value;

    listen("tauri://file-drop", (e) => {
      selectedFilePath = e.payload[0];
      selectedFilename = selectedFilePath.replace(/^.*[\\\/]/, "");
    });

    listen("file_selected", (e) => {
      selectedFilePath = e.payload as string;
      selectedFilename = selectedFilePath.replace(/^.*[\\\/]/, "");
    });

    // Reset the form when flashing is finished
    listen("flash_finished", () => {
      selectedPort = availablePorts[0].value;
      selectedFilename = "";
      selectedFilePath = "";
    });
  });

  function flashFirmware() {
    invoke("flash_firmware", {
      port: selectedPort,
      file: selectedFilePath,
    });
  }

  function flashImage() {
    invoke("flash_image", {
      port: selectedPort,
      file: selectedFilePath,
    });
  }

  export const flash = () => {
    console.log("flashType", flashType);
    console.log("selectedPort", selectedPort);
    console.log("selectedDeviceType", selectedDeviceType);

    if (flashType === "firmware") {
      flashFirmware();
    } else {
      flashImage();
    }
  };
</script>

<div class="p-5 flex flex-col gap-3">
  <div class="flex gap-3">
    <Label class="block space-y-2 flex-grow">
      <span>File</span>
      <Button
        btnClass="flex flex-col justify-center items-center bg-gray-50 w-full rounded-lg border border-gray-300 cursor-pointer"
        on:click={openDialog}
      >
        <p class="my-2 text-sm text-gray-500 cursor-pointer">
          {#if selectedFilename === ""}
            <span class="font-semibold">Click</span> or drag and drop
          {:else}
            <span class="text-blue-700">{selectedFilename}</span>
          {/if}
        </p>
      </Button>
    </Label>

    <Label class="block space-y-2">
      <span>Com port</span>
      <Select
        class="cursor-pointer"
        size="sm"
        items={availablePorts}
        bind:value={selectedPort}
      />
    </Label>
  </div>

  <Label class="block space-y-2">
    <span>Device type</span>
    <Select
      class="cursor-pointer"
      size="sm"
      items={[{ value: "8266", name: "Esp 8266" }]}
      bind:value={selectedDeviceType}
    />
  </Label>

  <Label class="block space-y-2">
    <span>Flash type</span>
    <ul
      class="flex items-center w-full rounded-lg border border-gray-400/70 divide-x divide-gray-400/70"
    >
      <li class="w-full">
        <Radio
          name="flash-type"
          class="p-2"
          value="firmware"
          bind:group={flashType}
        >
          Firmware
        </Radio>
      </li>
      <li class="w-full">
        <Radio
          name="flash-type"
          class="p-2"
          value="filesystem"
          bind:group={flashType}
        >
          Filesystem
        </Radio>
      </li>
    </ul>
  </Label>
</div>
