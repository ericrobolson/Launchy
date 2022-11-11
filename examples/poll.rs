use launchy::Pad;
use std::time::{Duration, Instant};

fn main() {
    let mut pad = Pad::new();

    let start = Instant::now();
    let max_duration = Duration::from_secs_f32(10.0);

    while (Instant::now() - start) < max_duration {
        if let Some(event) = pad.poll() {
            println!("{:?}", event)
        }
    }
}
