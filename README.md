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

## Example use case

- using [mods](https://github.com/charmbracelet/mods)
- using [extract]()

```bash
function summ() {
  /path/to/spinner/executable &

  # Capture the PID of the spinner
  local spinner_pid=$!

  # use mods for command line LLM wizadry
  # paste the content to summarize
  pbpaste | mods -f markdown --role summarizer --no-cache | bat -p -l md

  # Kill the spinner once the task is done
  kill $spinner_pid 2>/dev/null

}

```
