# Esp Flasher

This app allow beginner user to flash firmware and image on esp devices without installing ide and some complicated software.

:warning: Currently tested on a wemos d1 device

## How do use this app

1. Download this app and [esptool.exe](https://github.com/espressif/esptool/releases)
2. Put it together in a folder of your choice
3. Put is the firmware/image bin file in this same folder
   - Make sure the firmware bin file is named "firmware.bin"
   - Make sure the image bin file is named "image.bin"
4. Run EspFlasher and choose the com port of your board
5. Click on the button corresponding to what you want to do (flashing firmware or file image)
6. Wait until he finish
7. Et voila !

### TODO

- [ ] Allow users to choose what esp devices to flash and adapt the flash address
- [ ] Allow users to set manually flash address and other params like baudrate
- [ ] Allow users to choose what file to flash
- [ ] Maybe allowing devloppers to create a config file that imported by users ?
- [ ] Maybe a advanced mode ?
