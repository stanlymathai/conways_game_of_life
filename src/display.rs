use crate::cell::Cell;
use crate::universe::Universe;
use std::fmt::{Display as FmtDisplay, Formatter, Result};

// A text-based display for the universe.
pub struct Display<'a> {
    pub universe: &'a Universe,
}

impl<'a> Display<'a> {
    pub fn new(universe: &'a Universe) -> Display<'a> {
        Display { universe }
    }
}

impl<'a> FmtDisplay for Display<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for row in 0..self.universe.height {
            for col in 0..self.universe.width {
                let idx = self.universe.get_index(row, col);
                let symbol = match self.universe.cells[idx] {
                    Cell::Dead => '.',
                    Cell::Alive => '◼',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
