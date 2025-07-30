# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rotire is a command-line file rotation tool written in Rust. It archives or deletes old files in a directory while keeping the most recent N files (default: 4). Files can be archived into timestamped tar.gz files or permanently deleted.

## Commands

### Build and Run
- `cargo build` - Build the project
- `cargo run -- --help` - Show help and available options
- `cargo run -- --directory /path/to/dir` - Run with default archive action
- `cargo run -- --directory /path/to/dir delete` - Run with delete action

### Development
- `cargo check` - Quick compile check without producing binary
- `cargo clippy` - Run linter for code quality checks
- `cargo fmt` - Format code according to Rust style guidelines
- `cargo test` - Run tests (if any exist)

### Release
- `cargo build --release` - Build optimized release binary

## Architecture

The project follows a modular structure:

### Core Components
- **`main.rs`** - CLI argument parsing using clap, coordinates the main workflow
- **`rotire/mod.rs`** - Main `Rotire` struct and core business logic for file operations
- **`rotire/model.rs`** - Data structures (`RotireFile`, `RotireResult`) 
- **`rotire/filter.rs`** - File filtering system supporting prefix/suffix matching

### Key Concepts
- **RotireAction**: Enum defining operations (Archive, Delete)
- **RotireFilter**: Enum for file filtering (Prefix, Suffix)
- **File Processing**: Files are sorted by modification time, keeping the N newest files
- **Archive Format**: Creates timestamped tar.gz files in the target directory
- **Atomic Operations**: Uses AtomicBool to prevent concurrent runs

### Dependencies
- `clap` - Command-line argument parsing with derive macros
- `tar` + `flate2` - Archive creation and compression
- `anyhow` - Error handling
- `log` + `env_logger` - Logging infrastructure

## Usage Patterns

The tool operates on a directory, applying filters (optional) and actions:
1. Lists files in target directory
2. Applies prefix/suffix filters if specified  
3. Sorts files by modification time (oldest first)
4. Skips the N newest files (keep_n parameter)
5. Archives remaining files into tar.gz or deletes them

Example usage from README:
```bash
rotire --directory /home/denis/Pictures/not_in_train
rotire --directory /path --keep-n 10 --prefix-filter "backup-" delete
```