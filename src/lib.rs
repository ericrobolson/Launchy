mod launchpadx;
mod math;
mod types;

pub use types::{Led, PadEvent, PadEventType};

/// Interface for interacting with a Launchpad.
pub struct Pad {
    pad: launchpadx::LaunchpadX,
}
impl Pad {
    /// Creates a new instance of the pad.
    pub fn new() -> Self {
        Self {
            pad: launchpadx::LaunchpadX::new(),
        }
    }

    /// Polls the pad to get the most recent events. Typical usage is to call until nothing is returned.
    pub fn poll(&mut self) -> Option<PadEvent> {
        self.pad.poll()
    }

    /// Returns the width of the pad.
    pub fn width(&self) -> usize {
        self.pad.width()
    }

    /// Returns the height of the pad.
    pub fn height(&self) -> usize {
        self.pad.height()
    }

    /// Queues up the given command for the given LED.
    pub fn queue_led(&mut self, x: usize, y: usize, led: Led) {
        self.pad.set_led(x, y, led)
    }

    /// Flushes all pending LED operations.
    pub fn flush(&mut self) {
        self.pad.flush();
    }
}
