### TMP36 temperature sensor on Arduino 33 Iot with RTIC

Measures the temperature with TMP36 sensor every second using ADC, outputs to serial. 

At the moment uses the custom Arduino 33 Iot crate, as the one on crates.io doesn't have UART yet (AFAIK).

Also ADC is at the moment an unproven feature, therefore needs to be compiled with:

```bash
cargo build --release --features=unproven
```

You'll need to connect the board to an FTDI-USB board to read the serial output. 
Picocom is a light and simple serial terminal that can be used here.
