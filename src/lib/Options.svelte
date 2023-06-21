<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button, Label, Radio, Select } from "flowbite-svelte";
  import { onMount } from "svelte";
  import devices from "../utils/devices";
  import { deviceType, file, port, flashType } from "../utils/store";

  let availablePorts: Array<{ value: string; name: string }> = [];
  let availableDevices: Array<{ value: string; name: string }> = [];
  let filename = "";

  const openDialog = () => invoke("open_file_dialog");

  onMount(async () => {
    let ports: string[] = await invoke("list_ports");
    availablePorts = ports.map((port) => ({ value: port, name: port }));
    port.set(availablePorts[0].value);

    availableDevices = Object.keys(devices).map((key) => ({
      value: key,
      name: devices[key].name,
    }));

    listen("tauri://file-drop", (e) => {
      console.log(e.payload);
      file.set(e.payload[0]);
      filename = e.payload[0].replace(/^.*[\\\/]/, "");
    });

    listen("file_selected", (e) => {
      file.set(e.payload as string);
      console.log(e.payload);
      filename = $file.replace(/^.*[\\\/]/, "");
    });

    // Reset the form when flashing is finished
    listen("flash_finished", () => {
      port.set(availablePorts[0].value);
      file.set("");
    });
  });
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
          {#if $file === ""}
            <span class="font-semibold">Click</span> or drag and drop
          {:else}
            <span class="text-blue-700">{filename}</span>
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
        bind:value={$port}
      />
    </Label>
  </div>

  <Label class="block space-y-2">
    <span>Device type</span>
    <Select
      class="cursor-pointer"
      size="sm"
      items={availableDevices}
      bind:value={$deviceType}
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
          bind:group={$flashType}
        >
          Firmware
        </Radio>
      </li>
      <li class="w-full">
        <Radio
          name="flash-type"
          class="p-2"
          value="filesystem"
          bind:group={$flashType}
        >
          Filesystem
        </Radio>
      </li>
    </ul>
  </Label>
</div>
