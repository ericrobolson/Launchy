# Launchy
A Rust crate used for interacting with Launchpads.

# Usage
```
let mut pad = Pad::new();

// Queue up LEDs
pad.queue_led(8, 8, Led::Rgb((255, 255, 255)));
pad.queue_led(7, 7, Led::Rgb((255, 255, 0)));

// Flush the pad to set the values
pad.flush();

// Poll events
if let Some(event) = pad.poll() {
    println!("{:?}", event)
}

```


See `examples/poll.rs` and `examples/set_led.rs` for code you can run.


# Compatibility
- This has only been verified on Windows and M1 Macs. Linux and other platforms should be doable, but I haven't tested them out. Open an issue if one of these platforms ends up being supported and I will update the README.
- Only Launchpad X has been implemented as that is the only one I own.