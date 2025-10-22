# CLI Utils - Comprehensive Utility Library

A comprehensive Rust library providing essential utility functions for command-line applications and general programming tasks.

## Features

This library includes four main utility modules:

1. **Math Utils** - Mathematical operations and calculations
2. **String Utils** - String manipulation and analysis
3. **Date Utils** - Date and time operations
4. **File I/O Utils** - File system operations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cli-utils = "0.1.1"
chrono = { version = "0.4", features = ["serde"] }
```

## Usage

### Math Utils

```rust
use cli_utils::math_utils;

// Calculate factorial
let result = math_utils::factorial(5); // 120

// Find Greatest Common Divisor
let gcd = math_utils::gcd(48, 18); // 6

// Check if number is prime
let is_prime = math_utils::is_prime(17); // true

// Calculate Least Common Multiple
let lcm = math_utils::lcm(4, 6); // 12
```

### String Utils

```rust
use cli_utils::string_utils;

// Check palindrome
let is_palindrome = string_utils::is_palindrome("racecar"); // true

// Count characters
let count = string_utils::count_char("hello world", 'l'); // 3

// Reverse string
let reversed = string_utils::reverse_string("hello"); // "olleh"

// Convert to title case
let title = string_utils::to_title_case("hello world"); // "Hello World"

// Remove whitespace
let clean = string_utils::remove_whitespace("hello world"); // "helloworld"

// Count words
let words = string_utils::word_count("hello world"); // 2

// Check if alphabetic
let is_alpha = string_utils::is_alphabetic("hello"); // true
```

### Date Utils

```rust
use cli_utils::date_utils;

// Calculate date difference
let diff = date_utils::date_difference_days("2023-01-10", "2023-01-05")?; // 5

// Validate date format
let valid = date_utils::validate_date_format("2023-12-25", "%Y-%m-%d"); // true

// Format dates
let formatted = date_utils::format_date("2023-12-25", "%Y-%m-%d", "%d/%m/%Y")?; // "25/12/2023"

// Convert to different formats
let dd_mm_yyyy = date_utils::to_dd_mm_yyyy("2023-12-25")?; // "25/12/2023"
let yyyy_mm_dd = date_utils::to_yyyy_mm_dd("25/12/2023")?; // "2023-12-25"

// Get current date
let today = date_utils::current_date("%Y-%m-%d");

// Add/subtract days
let future = date_utils::add_days("2023-12-25", 7)?; // "2024-01-01"

// Check leap year
let is_leap = date_utils::is_leap_year(2024); // true

// Get day of week
let day = date_utils::day_of_week("2023-12-25")?; // "Monday"
```

### File I/O Utils

```rust
use cli_utils::file_io_utils;

// Read file to string
let content = file_io_utils::read_file_to_string("example.txt")?;

// Write string to file
file_io_utils::write_string_to_file("example.txt", "Hello, World!")?;

// Append to file
file_io_utils::append_to_file("example.txt", "\nNew line")?;

// Read file lines
let lines = file_io_utils::read_lines("example.txt")?;

// Write lines to file
let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
file_io_utils::write_lines("example.txt", &lines)?;

// Check if file exists
let exists = file_io_utils::file_exists("example.txt");

// Get file size
let size = file_io_utils::file_size("example.txt")?;

// Create directories
file_io_utils::create_dir_all("path/to/directory")?;

// Copy file
let bytes_copied = file_io_utils::copy_file("source.txt", "dest.txt")?;

// Delete file
file_io_utils::delete_file("unwanted.txt")?;

// Read first N lines
let first_lines = file_io_utils::read_first_n_lines("example.txt", 5)?;
```

## API Documentation

### Math Utils Functions

- `factorial(n: u64) -> u64` - Calculates factorial (max input: 20)
- `gcd(a: u64, b: u64) -> u64` - Greatest Common Divisor using Euclid's algorithm
- `is_prime(n: u64) -> bool` - Prime number checker with optimized algorithm
- `lcm(a: u64, b: u64) -> u64` - Least Common Multiple calculation

### String Utils Functions

- `is_palindrome(s: &str) -> bool` - Checks if string reads same forwards/backwards
- `count_char(s: &str, target: char) -> usize` - Counts character occurrences
- `reverse_string(s: &str) -> String` - Reverses string character order
- `to_title_case(s: &str) -> String` - Converts to title case
- `remove_whitespace(s: &str) -> String` - Removes all whitespace
- `word_count(s: &str) -> usize` - Counts words in string
- `is_alphabetic(s: &str) -> bool` - Checks if string contains only letters

### Date Utils Functions

- `date_difference_days(date1: &str, date2: &str) -> Result<i64, chrono::ParseError>` - Calculate day difference
- `validate_date_format(date_str: &str, format: &str) -> bool` - Validate date format
- `format_date(date_str: &str, input_format: &str, output_format: &str) -> Result<String, chrono::ParseError>` - Convert date formats
- `to_dd_mm_yyyy(date_str: &str) -> Result<String, chrono::ParseError>` - Convert to DD/MM/YYYY
- `to_yyyy_mm_dd(date_str: &str) -> Result<String, chrono::ParseError>` - Convert to YYYY-MM-DD
- `current_date(format: &str) -> String` - Get current date in specified format
- `add_days(date_str: &str, days: i64) -> Result<String, chrono::ParseError>` - Add/subtract days
- `is_leap_year(year: i32) -> bool` - Check if year is leap year
- `day_of_week(date_str: &str) -> Result<String, chrono::ParseError>` - Get day of week

### File I/O Utils Functions

- `read_file_to_string<P: AsRef<Path>>(file_path: P) -> io::Result<String>` - Read entire file
- `write_string_to_file<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()>` - Write/overwrite file
- `append_to_file<P: AsRef<Path>>(file_path: P, content: &str) -> io::Result<()>` - Append to file
- `read_lines<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<String>>` - Read file as lines
- `write_lines<P: AsRef<Path>>(file_path: P, lines: &[String]) -> io::Result<()>` - Write lines to file
- `file_exists<P: AsRef<Path>>(file_path: P) -> bool` - Check file existence
- `file_size<P: AsRef<Path>>(file_path: P) -> io::Result<u64>` - Get file size in bytes
- `create_dir_all<P: AsRef<Path>>(dir_path: P) -> io::Result<()>` - Create directories recursively
- `copy_file<P: AsRef<Path>>(source: P, destination: P) -> io::Result<u64>` - Copy file
- `delete_file<P: AsRef<Path>>(file_path: P) -> io::Result<()>` - Delete file
- `read_first_n_lines<P: AsRef<Path>>(file_path: P, n: usize) -> io::Result<Vec<String>>` - Read first N lines

## Examples

Run the usage demo:

```bash
cargo run --example usage_demo
```

## Testing

Run all tests:

```bash
cargo test
```

The library includes comprehensive unit tests and integration tests for all modules.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.