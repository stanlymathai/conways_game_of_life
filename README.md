# Conway's Game of Life

A Rust implementation of Conway's Game of Life, featuring a terminal-based visualization with animated glider patterns.

## About Conway's Game of Life

Conway's Game of Life is a cellular automaton devised by mathematician John Horton Conway in 1970. It's a zero-player game, meaning its evolution is determined by its initial state, requiring no further input. The game consists of a grid of cells that can be either alive or dead, and evolves according to simple rules.

### Rules

The universe evolves in discrete time steps according to these rules:

1. **Underpopulation**: Any live cell with fewer than 2 live neighbors dies
2. **Survival**: Any live cell with 2 or 3 live neighbors survives to the next generation
3. **Overpopulation**: Any live cell with more than 3 live neighbors dies
4. **Reproduction**: Any dead cell with exactly 3 live neighbors becomes alive

## Features

- 🦀 **Written in Rust** - Fast, safe, and memory-efficient implementation
- 🎮 **Terminal Animation** - Real-time visualization in your terminal
- 🛸 **Glider Pattern** - Pre-seeded with the famous glider pattern
- 🔄 **Toroidal Universe** - Edges wrap around (infinite grid simulation)
- ⚡ **Optimized** - Efficient neighbor counting and state transitions

## Project Structure

```
src/
├── main.rs      # Entry point and game loop
├── lib.rs       # Library module declarations
├── universe.rs  # Universe struct and game logic
├── cell.rs      # Cell state enum
└── display.rs   # Terminal display implementation
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (2024 edition or later)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd conways_game_of_life
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the simulation:
   ```bash
   cargo run
   ```

### Usage

When you run the program, you'll see a 30x30 grid in your terminal with a glider pattern that moves diagonally across the screen. The simulation runs continuously with a 200ms delay between generations.

- **Live cells** are represented by `◼` (black squares)
- **Dead cells** are represented by `.` (dots)

Press `Ctrl+C` to stop the simulation.

## Implementation Details

### Core Components

#### `Cell` Enum
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}
```

#### `Universe` Struct
- Manages the grid dimensions and cell states
- Implements Conway's rules in the `tick()` method
- Uses a flat `Vec<Cell>` for efficient memory layout
- Provides toroidal topology (edges wrap around)

#### `Display` Struct
- Implements `std::fmt::Display` for terminal output
- Renders the universe as ASCII characters

### Key Features

- **Efficient Neighbor Counting**: Uses modular arithmetic for edge wrapping
- **Double Buffering**: Creates a new state vector each tick to avoid conflicts
- **Memory Efficient**: Uses a flat vector instead of 2D array
- **Clear Terminal**: Automatically clears screen between frames for smooth animation

## The Glider Pattern

The simulation starts with a glider pattern - one of the most famous patterns in Conway's Game of Life:

```
. ◼ .
. . ◼
◼ ◼ ◼
```

This 5-cell pattern moves diagonally across the grid, returning to its original shape every 4 generations but shifted by one cell diagonally.

## Building and Testing

```bash
# Build the project
cargo build

# Run with optimizations
cargo run --release

# Run tests (if any)
cargo test

# Check code formatting
cargo fmt

# Run clippy for linting
cargo clippy
```

## Customization

You can easily modify the simulation by editing `src/main.rs`:

- **Grid Size**: Change `width` and `height` variables
- **Animation Speed**: Adjust the `Duration::from_millis()` value
- **Initial Pattern**: Replace the glider coordinates with other patterns
- **Display Characters**: Modify the symbols in `src/display.rs`

### Example: Adding a Different Pattern

Replace the glider pattern in `main.rs` with a blinker:

```rust
// Blinker pattern (oscillates between horizontal and vertical line)
let blinker = [(1, 1), (1, 2), (1, 3)];
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgments

- John Horton Conway for creating the Game of Life
- The Rust community for excellent documentation and tools
- [LifeWiki](https://conwaylife.com/) for pattern references and inspiration
