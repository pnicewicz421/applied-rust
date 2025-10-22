//! Usage examples for the utility libraries
//! 
//! This example demonstrates how to use each of the utility libraries

use cli_utils::{math_utils, string_utils, date_utils, file_io_utils};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CLI Utils Library Examples ===\n");

    // Math Utils Examples
    println!("📊 Math Utils:");
    println!("  Factorial of 5: {}", math_utils::factorial(5));
    println!("  GCD of 48 and 18: {}", math_utils::gcd(48, 18));
    println!("  Is 17 prime? {}", math_utils::is_prime(17));
    println!("  LCM of 4 and 6: {}", math_utils::lcm(4, 6));
    println!();

    // String Utils Examples
    println!("🔤 String Utils:");
    let test_string = "racecar";
    println!("  Is '{}' a palindrome? {}", test_string, string_utils::is_palindrome(test_string));
    println!("  Count of 'l' in 'hello world': {}", string_utils::count_char("hello world", 'l'));
    println!("  Reverse of 'hello': {}", string_utils::reverse_string("hello"));
    println!("  Title case of 'hello world': {}", string_utils::to_title_case("hello world"));
    println!("  Word count of 'hello world programming': {}", string_utils::word_count("hello world programming"));
    println!();

    // Date Utils Examples
    println!("📅 Date Utils:");
    println!("  Current date (YYYY-MM-DD): {}", date_utils::current_date("%Y-%m-%d"));
    println!("  Days between 2023-01-10 and 2023-01-05: {}", 
             date_utils::date_difference_days("2023-01-10", "2023-01-05")?);
    println!("  Is date '2023-12-25' valid in YYYY-MM-DD format? {}", 
             date_utils::validate_date_format("2023-12-25", "%Y-%m-%d"));
    println!("  Convert '2023-12-25' to DD/MM/YYYY: {}", 
             date_utils::to_dd_mm_yyyy("2023-12-25")?);
    println!("  Add 7 days to '2023-12-25': {}", 
             date_utils::add_days("2023-12-25", 7)?);
    println!("  Is 2024 a leap year? {}", date_utils::is_leap_year(2024));
    println!();

    // File I/O Utils Examples
    println!("📁 File I/O Utils:");
    let temp_file = "/tmp/cli_utils_example.txt";
    let sample_content = "Hello, World!\nThis is a sample file.\nCreated by cli-utils example.";
    
    // Write to file
    file_io_utils::write_string_to_file(temp_file, sample_content)?;
    println!("  ✓ Created file: {}", temp_file);
    
    // Check file exists and get size
    println!("  File exists? {}", file_io_utils::file_exists(temp_file));
    println!("  File size: {} bytes", file_io_utils::file_size(temp_file)?);
    
    // Read file content
    let content = file_io_utils::read_file_to_string(temp_file)?;
    println!("  File content preview: {}", content.lines().next().unwrap_or(""));
    
    // Read lines
    let lines = file_io_utils::read_lines(temp_file)?;
    println!("  Number of lines: {}", lines.len());
    
    // Append to file
    file_io_utils::append_to_file(temp_file, "\nAppended line!")?;
    println!("  ✓ Appended content to file");
    
    // Clean up
    file_io_utils::delete_file(temp_file)?;
    println!("  ✓ Cleaned up temporary file");
    
    println!("\n🎉 All utility libraries work correctly!");
    
    Ok(())
}