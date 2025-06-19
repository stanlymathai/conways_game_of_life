use crate::cell::Cell;

/// The Universe struct holds the dimensions and the grid of cells.
pub struct Universe {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

impl Universe {
    // Create a new universe with all cells dead.
    pub fn new(width: u32, height: u32) -> Self {
        let cells = vec![Cell::Dead; (width * height) as usize];
        Universe {
            width,
            height,
            cells,
        }
    }

    // Get the linear index for a cell at (row, column).
    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // Count live neighbors for a given cell.
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1] {
            for delta_col in [self.width - 1, 0, 1] {
                if delta_col == 0 && delta_row == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;

                let idx = self.get_index(neighbor_row, neighbor_col);
                if self.cells[idx] == Cell::Alive {
                    count += 1;
                }
            }
        }
        count
    }

    // Advance the unviverse by one tick according to Conway's rules.
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                next[idx] = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Dead, _) => Cell::Dead,
                    (Cell::Alive, _) => Cell::Alive,
                };
            }
        }
        self.cells = next;
    }
}
