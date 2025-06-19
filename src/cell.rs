/// Represents the state of a single cell in Conway’s Game of Life.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}
