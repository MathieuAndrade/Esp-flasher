<script lang="ts">
  import { Button } from "flowbite-svelte";

  import Footer from "./lib/Footer.svelte";
  import Titlebar from "./lib/Titlebar.svelte";
  import Options from "./lib/Options.svelte";
  import Progress from "./lib/Progress.svelte";

  let flash;
</script>

<main class="flex flex-col h-full bg-gray-200 rounded-lg">
  <Titlebar />

  <Options bind:flash />

  <div class="p-5 text-end">
    <Button
      pill={true}
      class="!px-2 h-[46px] w-[46px]"
      color="blue"
      on:click={flash}
    >
      Start
    </Button>
  </div>

  <Progress />

  <Footer />
</main>
