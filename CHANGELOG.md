# Changelog

## 0.1.0 (2026-09-08)

### Added

- Initialized the `booklog-cli` Rust project.
- Added CSV parsing with the `csv` crate.
- Added a `Book` struct to represent library loan records.
- Added conversion from CSV records to `Book` instances.
- Added CSV file path handling through command-line arguments.
- Added the `list` command to display borrowed book titles.
- Added command validation and error handling for missing or unknown arguments.
