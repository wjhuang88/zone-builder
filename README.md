# Zone Builder

Zone Builder is a Rust command-line application designed to process a specific format of blog article directories. It reads Markdown file content and metadata from a blog article directory, parses the frontmatter metadata, and updates JSON files in various directories accordingly.

## Features

- Parses TOML frontmatter from Markdown files (enclosed in `+++` delimiters)
- Updates category-specific `meta.json` files
- Updates root-level aggregation files (`latest.json`, `recommended.json`, `notebooks.json`, `index.json`)
- Handles recursive directory traversal
- Command-line interface for specifying blog directory path
- Modular architecture following Rust best practices

## Installation

Make sure you have Rust installed on your system. Then clone the repository and build:

```bash
git clone <repository-url>
cd zone-builder
cargo build --release
```

## Usage

Run the application with the path to your blog directory:

```bash
# Using cargo run
cargo run -- -p /path/to/blog/directory

# Or with the built binary
./target/release/zone-builder -p /path/to/blog/directory

# Or without specifying path (uses current directory by default)
cargo run
```

### Options

- `-p, --path <PATH>`: Path to the blog directory (default: ".")

## Testing

The project includes a sample test blog directory for testing purposes:

```bash
# Use the included test script
./test_samples.sh

# Or run directly with the sample directory
cargo run -- -p ./samples/test-blog
```

The sample test blog directory (`samples/test-blog`) contains example articles in different categories for testing the application.

## Blog Directory Structure

The tool processes blog directories with the following structure:

```
blog-directory/
├── <category>/                 # Category directories (e.g., tech, essay, demo)
│   ├── <article>.md           # Markdown files with TOML frontmatter
│   ├── images/                # Images directory
│   └── meta.json              # Metadata for articles in this category
├── latest.json                # Latest articles aggregated from all categories (limited to 5)
├── recommended.json           # Recommended articles (limited to 5)
├── notebooks.json             # Notebook entries with category information
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

## Architecture

The project follows a modular architecture:

- `src/lib.rs`: Library module declarations
- `src/article.rs`: Article parsing and processing logic
- `src/models.rs`: Data structure definitions
- `src/processor.rs`: Business logic and file processing
- `src/bin/main.rs`: Binary entry point

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architectural information.

## How It Works

1. **Directory Traversal**: The application walks through all markdown files in the blog directory structure
2. **Frontmatter Parsing**: It extracts metadata from the TOML frontmatter enclosed in `+++` delimiters
3. **JSON Updates**: It updates:
   - Category-specific `meta.json` files with articles from that category
   - Root-level files like `latest.json`, `recommended.json`, `notebooks.json`, and `index.json`
4. **Limiting**: `latest.json` and `recommended.json` are limited to 5 most recent articles
5. **Sorting**: Articles are sorted by date (newest first)

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Dependencies

- `serde` and `serde_json` for JSON parsing and serialization
- `toml` for TOML frontmatter parsing
- `walkdir` for recursive directory traversal
- `chrono` for date/time handling
- `clap` for command-line argument parsing

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite: `cargo test`
6. Submit a pull request

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.