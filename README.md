# Spinner

This project is a simple Rust application that demonstrates the use of a spinner animation in the terminal. It utilizes the `spinners` crate to provide a visual indication of a long-running process.

## Project Structure

- **src/main.rs**: The main Rust file containing the implementation of the spinner.
- **Cargo.toml**: The configuration file for the Rust project, specifying dependencies and metadata.

## Dependencies

The project relies on the following crate:

- `spinners` version 4.1.1: Used to create and manage spinner animations in the terminal.

## Usage

To run the project, ensure you have Rust and Cargo installed. Then, execute the following commands in your terminal:

```bash
cargo run
```

# Clone the repository

```bash
git clone https://github.com/yourusername/spinner.git

# Navigate to the project directory
cd spinner

# Build the project
cargo build

# Run the project
cargo run
```

## Code Overview

The main functionality is implemented in `src/main.rs`:

- **Concurrency**: The application uses Rust's threading capabilities to run the spinner in a separate thread, allowing the main program to perform other tasks concurrently.
- **Atomic Operations**: An `AtomicBool` wrapped in an `Arc` is used to safely share the spinner's running state across threads.
