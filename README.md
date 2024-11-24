# 🦈 Shark

**Shark** is a minimal shell written in Rust. It's a work-in-progress project created as a learning exercise to explore shell development and Rust programming.

## Features

- Tilde expansion: `~`
- Change directory: `cd`

## Installation

1. Install with cargo:
   ```bash
   cargo install --git=https://github.com/dybdeskarphet/shark
   shark
   ```
 
2. Clone and build:

   ```bash
   git clone https://github.com/dybdeskarphet/shark
   cd shark
   cargo build --release
   ./target/release/shark
   ```

## Future Plans

- [ ] Add support for more internal commands.
- [ ] Implement basic scripting capabilities.
- [ ] Add support for escape sequences.
- [ ] Improve error handling.

## License

This project is released under the [GNU GPLv3](LICENSE).
