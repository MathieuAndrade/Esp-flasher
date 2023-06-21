<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { AccordionItem, Input, Label, Select } from "flowbite-svelte";
  import { afterUpdate } from "svelte";
  import { baudrate, flashAddress } from "../utils/store";

  let optionsOpen = false;

  afterUpdate(() => {
    const button = document.querySelector("h2 button");

    if (button) {
      button.addEventListener("click", () => {
        optionsOpen = !optionsOpen;

        // Resize the window to fit the options
        invoke("set_window_size", {
          targetWidth: 400,
          targetHeight: optionsOpen ? 595 : 505,
          operationCount: 8,
        });
      });
    }
  });
</script>

<AccordionItem
  defaultClass="w-[-webkit-fill-available] flex items-center justify-between font-medium text-left border-b border-gray-400/70"
  paddingFlush="mx-5 py-2"
>
  <span slot="header"> Advanced options </span>

  <div slot="arrowup">
    <svg
      class="w-6 h-6 shrink-0 rotate-180"
      fill="currentColor"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M7.4 15.4L6 14l6-6l6 6l-1.4 1.4l-4.6-4.6l-4.6 4.6Z" />
    </svg>
  </div>
  <div slot="arrowdown">
    <svg
      class="w-6 h-6 shrink-0"
      fill="currentColor"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M7.4 15.4L6 14l6-6l6 6l-1.4 1.4l-4.6-4.6l-4.6 4.6Z" />
    </svg>
  </div>

  <div class="flex gap-3">
    <Label class="block space-y-2">
      <span>Baudrate</span>
      <Select
        class="cursor-pointer"
        size="sm"
        items={[
          { value: "921600", name: "921600" },
          { value: "57600", name: "57600" },
          { value: "256000", name: "256000" },
          { value: "512000", name: "512000" },
        ]}
        bind:value={$baudrate}
      />
    </Label>
    <div>
      <Label for="flashAddress" class="mb-2">Flash Address</Label>
      <Input
        type="text"
        id="flashAddress"
        placeholder="0x0"
        class="h-[38px]"
        bind:value={$flashAddress}
      />
    </div>
  </div>
</AccordionItem>
