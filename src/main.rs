use std::{thread, time::Duration};

use conways_game_of_life::cell::Cell;
use conways_game_of_life::display::Display;
use conways_game_of_life::universe::Universe;

fn main() {
    let width = 30;
    let height = 30;

    let mut universe = Universe::new(width, height);

    // Seed with a glider pattern
    let glider = [(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)];

    for (row, col) in glider.iter().cloned() {
        let idx = universe.get_index(row, col);
        universe.cells[idx] = Cell::Alive;
    }

    // Game loop
    loop {
        println!("{}", Display::new(&universe));
        universe.tick();

        thread::sleep(Duration::from_millis(200));
        print!("\x1B[2J\x1B[1;1H"); // clear screen
    }
}
