# Zone Builder - Development Guide for AI Agents

This project was developed using the Vibe Coding approach, emphasizing intuitive development with AI assistance for rapid prototyping and implementation.

## Project Overview

Zone Builder is a Rust command-line application designed to process a specific format of blog article directories. The primary purpose of this tool is to read Markdown file content and metadata from a blog article directory (like `/Users/GHuang/WorkSpace/BlogProjects/zone-articles`), parse the frontmatter metadata, and update JSON files in various directories accordingly.

The tool handles blog articles in TOML frontmatter format (enclosed in +++ delimiters) and updates corresponding meta.json files in each article directory as well as aggregated JSON files at the root level (latest.json, recommended.json, etc.).

## Project Structure

```
zone-builder/
├── Cargo.toml          # Project manifest and dependencies
├── .gitignore          # Git ignore rules
├── src/
│   └── main.rs         # Main application entry point
└── .git/               # Git repository metadata
```

## Build Configuration

The Vibe Coding approach emphasizes rapid iteration and testing:

- **Language**: Rust 2024 edition
- **Build System**: Cargo (Rust's package manager and build system)
- **Project Version**: 0.1.0
- **Target**: Command-line application for processing blog article directories

### Build Commands

```bash
# Build the project in development mode
cargo build

# Build the project in release mode
cargo build --release

# Check the code without producing output
cargo check

# Clean build artifacts
cargo clean
```

## Testing Configuration

The Vibe Coding approach emphasizes rapid iteration and testing:

- **Test Framework**: Built-in Rust testing framework
- **Test Location**: Tests can be added in `src/main.rs` or in `tests/` directory
- **Integration Tests**: Should test with sample blog directories like `/Users/GHuang/WorkSpace/BlogProjects/zone-articles`

### Test Commands

```bash
# Run all tests
cargo test

# Run tests with more verbose output
cargo test --verbose

# Run tests with specific filter
cargo test <test_name>

# Run specific integration test
cargo test --test <integration_test_name>
```

## Linting and Code Quality

The Vibe Coding approach encourages consistent code quality:

- **Linter**: Clippy (Rust's official linter)
- **Formatter**: rustfmt (Rust's official code formatter)

### Linting Commands

```bash
# Install clippy if not already installed
rustup component add clippy

# Run clippy to check for common mistakes and improvements
cargo clippy

# Run clippy with specific lint levels
cargo clippy -- -W clippy::all -D clippy::pedantic
```

### Formatting Commands

```bash
# Install rustfmt if not already installed
rustup component add rustfmt

# Format all code according to Rust style guidelines
cargo fmt

# Check if code is properly formatted without changing files
cargo fmt --check
```

## Dependencies Management

The Vibe Coding approach helped efficiently incorporate these dependencies, which are managed through `Cargo.toml`. Expected additions will likely include:

- `serde` and `serde_json` for JSON parsing and serialization
- `toml` for TOML frontmatter parsing
- `walkdir` for recursive directory traversal
- `chrono` for date/time handling
- `regex` for pattern matching
- `clap` for command-line argument parsing

Expected additions will likely include:
- `clap` = "4.0"  # for command-line argument parsing

```toml
[package]
name = "zone-builder"
version = "0.1.0"
edition = "2024"

[dependencies]
# Core dependencies for the blog processing functionality
# serde = { version = "1.0", features = ["derive"] }
# serde_json = "1.0"
# toml = "0.8"
# walkdir = "2.3"
# chrono = { version = "0.4", features = ["serde"] }
# clap = "4.0"  # for command-line argument parsing
```

## Blog Article Directory Structure

The tool processes blog directories with the following structure:

```
blog-directory/
├── <category>/                 # Category directories (e.g., tech, essay, demo)
│   ├── <article>.md           # Markdown files with TOML frontmatter
│   ├── images/                # Images directory
│   └── meta.json              # Metadata for articles in this category
├── latest.json                # Latest articles aggregated from all categories
├── recommended.json           # Recommended articles
├── notebooks.json             # Notebook articles
└── index.json                 # Main index of all articles
```

### Markdown File Format (TOML Frontmatter)

```toml
+++
title = "Article Title"
date = "YYYY-MM-DD"
update = "YYYY-MM-DD"
summary = "Brief description of the article"
path = "filename.md"
collection = "category"  # Optional: for root-level aggregation
+++

Article content goes here...
```

## Code Style Guidelines

The Vibe Coding approach helps maintain consistent code style and quality:

- Follow Rust standard formatting conventions (enforced by rustfmt)
- Follow Rust naming conventions (snake_case for variables/functions, PascalCase for types)
- Use Rust documentation comments (///) for public APIs
- Include tests for all non-trivial functionality
- Use Clippy to identify potential improvements and common mistakes
- Handle file I/O errors gracefully with appropriate error propagation
- Use proper logging for file processing status

## Development Workflow

The Vibe Coding approach emphasizes rapid iteration and intuitive development:

1. **Setup**: Install Rust toolchain with `rustup`
2. **Development**: Use `cargo check` for quick compilation checks
3. **Formatting**: Regularly run `cargo fmt` to maintain consistent code style
4. **Linting**: Run `cargo clippy` to catch potential issues
5. **Testing**: Write and run tests with `cargo test`, especially with sample blog directories
6. **Building**: Use `cargo build` for development builds or `cargo build --release` for optimized builds
7. **Integration Testing**: Test with `/Users/GHuang/WorkSpace/BlogProjects/zone-articles` directory

## Rust Toolchain Information

- **Rust Compiler**: rustc 1.92.0 (ded5c06cf 2025-12-08)
- **Cargo**: 1.92.0 (344c4567c 2025-10-21)
- **Edition**: 2024

## Git Configuration

- Target directory (`/target`) is ignored by git as per `.gitignore`
- Source code is tracked in the `src/` directory
- Configuration files like `Cargo.toml` and `Cargo.lock` are tracked

## Common Tasks for AI Agents

The Vibe Coding approach encourages AI agents to work with intuitive development cycles. When working with this codebase, AI agents should:

1. **Before modifying code**: Run `cargo fmt` to ensure proper formatting
2. **After implementing changes**: Run `cargo clippy` to check for improvements
3. **Before committing**: Run `cargo test` to ensure nothing is broken
4. **For dependency updates**: Modify `Cargo.toml` and run `cargo build` to resolve dependencies
5. **When adding functionality**: Focus on parsing TOML frontmatter, updating JSON files, and processing file hierarchies
6. **For testing**: Use sample data from `/Users/GHuang/WorkSpace/BlogProjects/zone-articles` to validate functionality

## Key Implementation Areas

The Vibe Coding approach facilitated rapid implementation of these components:

1. **File Processing**: Read Markdown files and parse TOML frontmatter
2. **Metadata Extraction**: Extract title, date, update, summary, path, collection from frontmatter
3. **Directory Traversal**: Walk through blog directory structure recursively
4. **JSON Serialization**: Update meta.json files in each category and root-level aggregation files
5. **Command-Line Interface**: Accept directory path as argument and process accordingly

## Future Considerations

As the project grows, consider adding:
- Command-line argument parsing with `clap`
- Proper error handling and logging
- Configuration file support for custom field mappings
- Incremental processing with change detection
- Backup mechanisms for JSON files before updates
- Validation of frontmatter fields
- Performance optimizations for large blog directories

The Vibe Coding approach encourages continued rapid iteration and AI-assisted development for implementing these features.
