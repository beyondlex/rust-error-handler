# AGENTS.md

## Build, Lint, and Test Commands

### Core Commands
- `cargo build` - Build the project
- `cargo run` - Build and run the main binary
- `cargo check` - Fast check for errors without building
- `cargo clippy` - Run linter to catch common mistakes
- `cargo fmt` - Format all code using rustfmt
- `cargo test` - Run all unit and integration tests
- `cargo doc --open` - Build and open documentation

### Watch Mode
- `cargo watch -q -c -x 'run -q'` - Watch for changes and run

### Running Specific Tests
- `cargo test <test_name>` - Run a specific test by name
- `cargo test -- <filter>` - Run tests matching a filter
- `cargo test <module_name>` - Run all tests in a module

## Code Style Guidelines

### Module Structure
- Declare modules at the top of files: `mod fs;`
- Re-export public types at crate level: `pub use error::{Error, Result};`
- Use lowercase with underscores for module names: `fs`, `error`

### Imports
- Group imports logically: std library, external crates, local modules
- Use `crate::` prefix for internal module paths: `crate::Result<T>`, `crate::fs::list_files`
- Prefer specific imports over glob imports where possible

### Error Handling
- Define Result type alias: `pub type Result<T> = core::result::Result<T, Error>;`
- Use derive_more with From feature for automatic error conversion: `#[derive(Debug, From)]`
- Use `#[from]` attribute for automatic From implementations
- Always implement `Display` for custom errors: `write!(f, "{self:?}")`
- Implement `std::error::Error` trait for all custom errors
- Use `?` operator for error propagation
- Return errors using `Error::Variant` pattern

### Types and Enums
- Use PascalCase for types and enum variants
- Derive `Debug` for custom types
- Use `From` derive for automatic conversions between error types
- Enum variants should be descriptive: `SillyOneCantListEmptyFolder`

### Naming Conventions
- Functions: snake_case (`list_files`, `main`)
- Variables: snake_case (`path`, `files`)
- Types/Enums: PascalCase (`Error`, `Result`)
- Constants: SCREAMING_SNAKE_CASE (when needed)
- Private module types can be snake_case variants

### Code Organization
- Module files in `src/module_name/mod.rs` or `src/module_name.rs`
- Keep related enums and impls in the same file
- Use blank lines to separate logical sections
- End files with a trailing newline

### Function Design
- Use `&str` for string arguments when ownership isn't needed
- Return `crate::Result<T>` for fallible operations
- Chain operations with filter_map/filter for cleaner code
- Handle empty collections explicitly when needed

### Comments
- Keep comments minimal
- Use TODO comments for future work
- Focus on "why" rather than "what"
- Remove commented-out code

### Error Pattern Template
```rust
use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Io(std::io::Error),
    CustomErrorType
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
```

### Testing
- Use `#[test]` attribute for test functions
- Place tests in `tests` module at bottom of file: `#[cfg(test)] mod tests { ... }`
- Use descriptive test names that describe what is being tested
- Assert expected behavior with `assert!` or `assert_eq!`

### Formatting
- Use rustfmt for consistent formatting
- Keep lines under 100 characters when possible
- Use 4 spaces for indentation (rustfmt default)
