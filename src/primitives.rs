use crate::math::{self, index_2d_to_1d};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub led: Led,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum Led {
    Off,
    Rgb((u8, u8, u8)),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
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

    /// Returns the number of items in the grid.
    pub fn count(&self) -> usize {
        self.values.len()
    }

    /// Sets the given value.
    pub fn set(&mut self, x: usize, y: usize, led: Led) {
        let idx = index_2d_to_1d(x, y, self.width, self.height);
        self.values[idx].led = led;
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        let idx = index_2d_to_1d(x, y, self.width, self.height);
        self.values[idx]
    }

    /// Returns the values in the grid.
    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        self.values.iter()
    }

    /// Returns the values in the grid.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.values.iter_mut()
    }
}
