# Ergot Demo

## About

A demo that uses [ergot] to communicate between a host PC and an [RP2040], to control a bunch of [WS2812] smart LEDs.

[ergot]: https://github.com/jamesmunns/ergot
[RP2040]: https://www.raspberrypi.com/products/rp2040/
[WS2812]: http://world-semi.com/ws2812-family/

## Usage

This demo consist of three crates:

- [`firmware`](firmware/), which runs on the RP2040.
- [`cli`](cli/), to communicate with the firmware from the host PC.
- [`shared`](shared/), a library with shared code between the two.

### Run the firmware

I've been writing this demo against a board that's not publicly available. If you have a suitable board with both an RP2040 and one or more WS2812, you should be able to adapt the firmware, by changing the pins and number of colors written.

Install [`picotool`](https://github.com/raspberrypi/picotool) and connect the RP2040 with the USB bootloader being active. Then go the the `firmware/` directory and execute `cargo run`:

```
cd firmware
cargo run
```

If everything works, the firmware will wait for commands from the host PC now.

### Change LED colors

Initially, the firmware will leave both LEDs off. To enable them and change your color, you can send commands using the CLI application.

In theory, you should be able to do that by going to the `cli/` directory and executing `cargo run` with the respective command. However, on Linux, you don't have permission to access arbitrary USB devices by default.

You either need the right `udev` configuration, or you can circumvent this by running the CLI app as the superuser:

```
cd cli
cargo build
sudo ./target/debug/cli
```

Only do this, if you're trusting the code!

Running the CLI app without arguments will do nothing. To have an effect, you need to pass an argument. For example, passing `--r1 255` will set the "red" channel of LED 1 to full intensity. Pass `--help` for a full list of arguments.

In theory, the CLI app should also work on macOS and Windows, but I haven't tested this.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
