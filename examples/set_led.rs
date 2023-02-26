use launchy::{Led, Pad};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut pad = Pad::new();

    let mut r: u8 = 0;

    for x in 0..pad.width() {
        for y in 0..pad.height() {
            r = r.wrapping_add(1);
            pad.queue_led(x, y, Led::Rgb((r, 0, 255 - r)));
        }
    }

    pad.queue_led(8, 8, Led::Rgb((255, 255, 255)));

    pad.flush();

    sleep(Duration::from_secs_f32(10.0));
}
