# GEMINI.md

## Project Overview

This project, "rotire", is a command-line file rotation tool written in Rust. It's designed to manage files in a directory by keeping a specified number of the newest files and archiving or deleting the older ones. This is useful for managing logs, backups, or any other directory where you want to keep a limited history of files.

The main technologies used are:
*   **Rust**: The core language for the project.
*   **clap**: For command-line argument parsing.
*   **tar** and **flate2**: For creating compressed tar archives.
*   **anyhow**: For error handling.
*   **log** and **env_logger**: For logging.

The project is structured into a `main.rs` for the CLI and a `rotire` module containing the core logic, data models, and filtering system.

## Building and Running

### Building the project

To build the project, you can use the following cargo commands:

*   **Debug build:**
    ```bash
    cargo build
    ```
*   **Release build (optimized):**
    ```bash
    cargo build --release
    ```
*   **Quick compile check:**
    ```bash
    cargo check
    ```

### Running the project

You can run the application using `cargo run` or by executing the compiled binary directly.

*   **Run with cargo:**
    ```bash
    cargo run -- --directory /path/to/test/dir
    ```
*   **Run built binary:**
    ```bash
    ./target/release/rotire --directory /path/to/test/dir
    ```

### Running tests

To run the test suite, use the following command:

```bash
cargo test
```

## Development Conventions

*   **Formatting**: The project uses `cargo fmt` for code formatting.
*   **Linting**: `cargo clippy` is used for linting the code.
*   **Error Handling**: The project uses the `anyhow` crate for simplified error handling.
*   **Logging**: The `log` and `env_logger` crates are used for logging.
*   **Contributions**: The `readme.md` file outlines a contribution process involving forking the repository, creating a feature branch, and submitting a pull request after running tests, formatting, and linting.
