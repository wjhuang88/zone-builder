# Zone Builder

Zone Builder is a Rust command-line application designed to process a specific format of blog article directories. It reads Markdown file content and metadata from a blog article directory, parses the frontmatter metadata, and updates JSON files in various directories accordingly.

**This project was developed using the Vibe Coding approach, emphasizing intuitive development with AI assistance for rapid prototyping and implementation.**

## Features

- Parses TOML frontmatter from Markdown files (enclosed in `+++` delimiters)
- Updates category-specific `meta.json` files
- Updates root-level aggregation files (`latest.json`, `recommended.json`, `notebooks.json`, `index.json`)
- Handles recursive directory traversal
- Command-line interface for specifying blog directory path
- Modular architecture following Rust best practices
- Developed using the Vibe Coding approach for rapid and intuitive implementation

## Installation

### Prerequisites

Make sure you have Rust installed on your system:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build and Install Local Executable

Clone the repository and build the release executable:

```bash
git clone <repository-url>
cd zone-builder
cargo build --release
```

The executable will be available at `target/release/zone-builder`.

### Install to System PATH (Optional)

To install the executable globally on your system:

```bash
# Copy the executable to a directory in your PATH
sudo cp target/release/zone-builder /usr/local/bin/

# Or copy to your local bin directory
mkdir -p ~/.local/bin
cp target/release/zone-builder ~/.local/bin/
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

This project showcases the effectiveness of the Vibe Coding approach, demonstrating how AI-assisted development can rapidly build and refine complex applications with proper architecture and testing.

## Blog Directory Structure

The tool processes blog directories with the following structure, developed using the Vibe Coding approach to ensure efficient handling of complex file structures:

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

The Vibe Coding approach enabled intuitive handling of the TOML frontmatter format:

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

The project follows a modular architecture, developed using Vibe Coding approach for rapid and intuitive implementation:

- `src/lib.rs`: Library module declarations
- `src/article.rs`: Article parsing and processing logic
- `src/models.rs`: Data structure definitions
- `src/processor.rs`: Business logic and file processing
- `src/bin/main.rs`: Binary entry point

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architectural information.

## How It Works

This project exemplifies the effectiveness of the Vibe Coding approach, where AI-assisted development enables rapid implementation of complex functionality. The application was built with intuitive development cycles, allowing for quick refinement of the core functionality:

1. **Directory Traversal**: The application walks through all markdown files in the blog directory structure
2. **Frontmatter Parsing**: It extracts metadata from the TOML frontmatter enclosed in `+++` delimiters
3. **JSON Updates**: It updates:
   - Category-specific `meta.json` files with articles from that category
   - Root-level files like `latest.json`, `recommended.json`, `notebooks.json`, and `index.json`
4. **Limiting**: `latest.json` and `recommended.json` are limited to 5 most recent articles
5. **Sorting**: Articles are sorted by date (newest first)

## Development

The project was developed using the Vibe Coding approach, emphasizing rapid development with AI assistance for efficient implementation. This methodology allowed for quick iteration and architectural refinement.

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Dependencies

The Vibe Coding approach helped efficiently incorporate these dependencies:

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

The Vibe Coding approach encourages rapid iteration and AI-assisted development for faster contributions. Feel free to leverage AI tools for code generation, refactoring, and testing to accelerate your development process.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

Built with the Vibe Coding approach, promoting open and collaborative development practices.