<script lang="ts">
  import { Accordion, Button } from "flowbite-svelte";

  import AdvancedOptions from "./lib/AdvancedOptions.svelte";
  import Footer from "./lib/Footer.svelte";
  import Options from "./lib/Options.svelte";
  import Progress from "./lib/Progress.svelte";
  import Titlebar from "./lib/Titlebar.svelte";

  let flash;
</script>

<main class="flex flex-col h-full bg-gray-200 rounded-lg overflow-hidden">
  <Titlebar />

  <Options bind:flash />

  <Accordion flush class="flex-grow">
    <AdvancedOptions />
  </Accordion>

  <div class="p-5 text-end">
    <Button pill={true} class="!px-2 h-[46px] w-[46px]" on:click={flash}>
      Start
    </Button>
  </div>

  <Progress />

  <Footer />
</main>
