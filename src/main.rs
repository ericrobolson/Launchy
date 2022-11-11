use primitives::Led;

mod launchpadx;
mod math;
mod primitives;

fn main() {
    let mut l = launchpadx::LaunchpadX::new();

    l.set_led(0, 0, Led::Rgb((255, 0, 255)));
    l.set_led(1, 1, Led::Rgb((0, 2, 2)));

    l.flush();

    let mut r = 0;
    let mut x = 0;
    let mut y = 0;

    loop {
        if let Some(event) = l.poll() {
            println!("{:?}", event);
        }

        if x > 8 {
            x = 0;
            y += 1;

            if y > 8 {
                y = 0;
            }
        }

        l.set_led(x, y, Led::Rgb((r, 0, 255)));
        x += 1;

        r = r.wrapping_add(1);
    }
}
