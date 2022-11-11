use crate::math::{self, index_2d_to_1d};

/// An event generated from a pad.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PadEvent {
    pub x: usize,
    pub y: usize,
    pub event: PadEventType,
}

/// The type of event that occured.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PadEventType {
    Pressed { velocity: u8 },
    Held { velocity: u8 },
    Released,
}

/// The state of a LED.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum Led {
    Off,
    Rgb((u8, u8, u8)),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Cell {
    pub x: usize,
    pub y: usize,
    pub led: Led,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Grid {
    width: usize,
    height: usize,
    values: Vec<Cell>,
}

impl Grid {
    /// Creates a new grid of LEDs.
    pub fn new(width: usize, height: usize) -> Self {
        let capacity = width * height;
        let mut values = Vec::with_capacity(capacity);
        for i in 0..capacity {
            let (x, y) = math::index_1d_to_2d(i, width, height);

            values.push(Cell {
                x,
                y,
                led: Led::Off,
            });
        }

        Self {
            values,
            width,
            height,
        }
    }

    /// Sets the given value.
    pub fn set(&mut self, x: usize, y: usize, led: Led) {
        let idx = index_2d_to_1d(x, y, self.width, self.height);
        self.values[idx].led = led;
    }

    /// Returns the item at the given cell.
    pub fn get(&self, x: usize, y: usize) -> Cell {
        let idx = index_2d_to_1d(x, y, self.width, self.height);
        self.values[idx]
    }

    /// Returns the values in the grid.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.values.iter_mut()
    }
}
