import { writable } from "svelte/store";

/** Reactive vars */

const port = writable("COM1");
const file = writable("");
const deviceType = writable("8266");
const flashType = writable("firmware");
const baudrate = writable("512000");
const flashAddress = writable("0x0");
const isFlashing = writable(false);

export {
  baudrate,
  deviceType,
  file,
  flashAddress,
  flashType,
  isFlashing,
  port,
};
