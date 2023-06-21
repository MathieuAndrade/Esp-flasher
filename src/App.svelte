<script lang="ts">
  import { Accordion, Button } from "flowbite-svelte";

  import AdvancedOptions from "./lib/AdvancedOptions.svelte";
  import Footer from "./lib/Footer.svelte";
  import Options from "./lib/Options.svelte";
  import Progress from "./lib/Progress.svelte";
  import Titlebar from "./lib/Titlebar.svelte";

  import { port, file, baudrate, flashAddress } from "./utils/store";

  export const flash = () => {
    console.log("flash", $port, $file, $baudrate, $flashAddress);

    /*
    invoke("flash", {
      port: $port,
      file: $file,
      baudrate: $baudrate,
      flashAddress: $flashAddress,
    });
    */
  };
</script>

<main class="flex flex-col h-full bg-gray-200 rounded-lg overflow-hidden">
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

  <Progress />

  <Footer />
</main>
