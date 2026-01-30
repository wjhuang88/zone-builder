# Zone Builder - Architecture

## Project Structure

The Zone Builder project follows Rust best practices with a modular structure:

```
zone-builder/
├── Cargo.toml          # Project manifest and dependencies
├── ARCHITECTURE.md     # This file - architectural overview
├── README.md           # Project documentation
├── src/
│   ├── lib.rs          # Library module declarations
│   ├── article.rs      # Article parsing and processing logic
│   ├── models.rs       # Data structure definitions
│   ├── processor.rs    # Business logic and file processing
│   └── bin/
│       └── main.rs     # Binary entry point
└── tests/
    └── integration_tests.rs  # Integration tests
```

## Module Breakdown

### `lib.rs`
Main library module that exports all public components.

### `article.rs`
- Defines the `Article` struct
- Contains frontmatter parsing logic
- Handles extraction of metadata from Markdown files

### `models.rs`
- Data structure definitions for JSON formats
- `IndexJson` - Structure for index.json with meta and list
- `MetaInfo` - Metadata information structure
- `NotebookEntry` - Structure for notebook entries

### `processor.rs`
- Core business logic for processing blog directories
- `BlogProcessor` struct with all processing methods
- Handles updating category meta.json files
- Manages root JSON file updates (latest.json, recommended.json, etc.)
- Implements directory traversal and file processing

### `src/bin/main.rs`
- Binary entry point
- Command-line argument parsing
- Application initialization

## Key Features

1. **Modular Design**: Clear separation of concerns between data structures, parsing logic, and processing logic
2. **Scalable**: Easy to add new functionality to specific modules
3. **Testable**: Each module can be tested independently
4. **Maintainable**: Well-defined interfaces between components

## Dependencies

- `clap`: Command-line argument parsing
- `serde`/`serde_json`: JSON serialization/deserialization
- `toml`: TOML frontmatter parsing
- `walkdir`: Recursive directory traversal
- `chrono`: Date/time handling