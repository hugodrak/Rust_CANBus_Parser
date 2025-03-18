
# Rust-CANBus-Parser

## Overview
A fast and memory-safe Rust library for parsing CAN bus messages in real-time environments.

## Features
- **High Performance**: Utilizes Rust’s ownership model for safe concurrency.
- **Modular Design**: Separation of parsing logic and error handling.
- **Scalable**: Easily extend message definitions for automotive applications.

## Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
rust-canbus-parser = { git = "https://github.com/YourGitHubUsername/Rust-CANBus-Parser.git" }
```

Then in your Rust code:
```rust
use rust_canbus_parser::CanParser;

fn main() {
    let parser = CanParser::new();
    // ...
}
```

# How to Build & Run

1. **Navigate** to the project root (`rust-canbus-parser/`).
2. **Build** the project:
   ```bash
   cargo build
   ```
3. **Run** the example:
   ```bash
   cargo run
   ```
4. **Test** the library (unit tests in `parser.rs`):
   ```bash
   cargo test
   ```

You should see the parsed message printed to your console if everything is configured correctly.

---

## Further Ideas

- **Extended CAN IDs**: In reality, extended CAN IDs can be 29 bits. You can update your parser logic to handle that.  
- **Real I/O**: Integrate with a CAN device or a socket (e.g., SocketCAN on Linux) to test real-time frames.  
- **Serialization**: Add (de)serialization using `serde` if you need to convert structures to JSON or other formats.  
- **Performance Optimizations**: Explore zero-copy parsing or specialized data structures if performance is critical.  