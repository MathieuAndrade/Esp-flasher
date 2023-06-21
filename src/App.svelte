<script lang="ts">
  import { Accordion, Button } from "flowbite-svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  import AdvancedOptions from "./lib/AdvancedOptions.svelte";
  import Footer from "./lib/Footer.svelte";
  import Options from "./lib/Options.svelte";
  import Progress from "./lib/Progress.svelte";
  import Titlebar from "./lib/Titlebar.svelte";

  import {
    port,
    file,
    baudrate,
    flashAddress,
    isFlashing,
  } from "./utils/store";

  export const flash = () => {
    invoke("flash", {
      port: $port,
      file: $file,
      baudRate: Number($baudrate),
      flashAddress: Number($flashAddress),
    });
  };
</script>

<main class="flex flex-col h-full bg-gray-200 rounded-lg overflow-hidden">
  <div class={$isFlashing && "blur-[1px]"}>
    <Titlebar />

    <Options />

    <Accordion flush class="flex-grow">
      <AdvancedOptions />
    </Accordion>

    <div class="p-5 text-end">
      <Button
        pill={true}
        class="!px-2 h-[46px] w-[46px]"
        on:click={flash}
        disabled={$file === ""}
      >
        Start
      </Button>
    </div>

    <Footer />
  </div>
  <Progress />
</main>
