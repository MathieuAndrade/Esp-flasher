<script lang="ts">
  import Footer from "./lib/Footer.svelte";
  import Form from "./lib/From.svelte";
  import Header from "./lib/Header.svelte";
  import Progress from "./lib/Progress.svelte";
  import Stepper from "./lib/Stepper.svelte";

  let step = 0;
</script>

<main class="flex flex-col h-full items-center">
  <Header />
  <div class="grow p-4 w-[80%] pt-6">
    <Stepper {step} />
    <Form bind:step />
    <Progress />
  </div>
  <Footer />
</main>
