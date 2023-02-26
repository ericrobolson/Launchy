use launchy::{Led, Pad};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut pad = Pad::new();

    let mut c = 0;

    for x in 0..pad.width() {
        for y in 0..pad.height() {
            pad.queue_led(x, y, Led::Rgb((c, 0, 0)));
            c = c.wrapping_add(1);
        }
    }

    for y in 0..pad.height() {
        pad.queue_led(8, y, Led::Rgb((0, 255, 0)));
    }

    for x in 0..pad.width() {
        pad.queue_led(x, 8, Led::Rgb((0, 0, 255)));
    }

    pad.queue_led(8, 8, Led::Rgb((255, 255, 255)));

    pad.flush();

    sleep(Duration::from_secs_f32(10.0));
}
