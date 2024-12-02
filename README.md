# Advent of Code on ESP32

A slightly ambitious attempt at completing advent of code 2024 on an ESP32, within the time constraints of serving results to a web page. This project is configured for an ESP32-C3-Mini as this is all I had on hand, but there's no reason you couldn't adapt this project for other microcontrollers. 

Presents an HTTP server where you can provide inputs and it will attempt to solve them. 

# Running the example

Expects the env variables WIFI_SSID and WIFI_PASSWORD to be set to the credentials for an appropriate nearby access point. You can also add these as `KEY=VAR` entries in a `.env` file in the root of the project. 

You may need to adjust the security scheme in code to match that used by the AP (default is WPA2).

It doesn't implement mdns so you're going to need a way of figuring out its IP address. This is reported over the serial port, or you can look at your router.

The project supports flashing via `cargo espflash`: 

```
cargo espflash flash --monitor --release -L defmt
```

# Demo video

Coming soon (have to upload through the online editor).