use cli_utils::{math_utils, string_utils, date_utils, file_io_utils, read_stdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to CLI Utils!");
    println!("This is a utility library with math, string, date, and file operations.");
    println!();
    
    loop {
        println!("Choose an option:");
        println!("1. Math Utils Demo");
        println!("2. String Utils Demo");
        println!("3. Date Utils Demo");
        println!("4. File I/O Utils Demo");
        println!("5. Interactive Mode");
        println!("6. Exit");
        println!();
        print!("Enter your choice (1-6): ");
        
        let input = read_stdin();
        
        match input.trim() {
            "1" => math_demo(),
            "2" => string_demo(),
            "3" => date_demo()?,
            "4" => file_demo()?,
            "5" => interactive_mode(),
            "6" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice. Please enter 1-6."),
        }
        println!();
    }
    
    Ok(())
}

fn math_demo() {
    println!("=== Math Utils Demo ===");
    println!("Factorial of 5: {}", math_utils::factorial(5));
    println!("GCD of 48 and 18: {}", math_utils::gcd(48, 18));
    println!("Is 17 prime? {}", math_utils::is_prime(17));
    println!("LCM of 4 and 6: {}", math_utils::lcm(4, 6));
}

fn string_demo() {
    println!("=== String Utils Demo ===");
    let test_string = "racecar";
    println!("Is '{}' a palindrome? {}", test_string, string_utils::is_palindrome(test_string));
    println!("Count of 'l' in 'hello world': {}", string_utils::count_char("hello world", 'l'));
    println!("Reverse of 'hello': {}", string_utils::reverse_string("hello"));
    println!("Title case of 'hello world': {}", string_utils::to_title_case("hello world"));
}

fn date_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Date Utils Demo ===");
    println!("Current date: {}", date_utils::current_date("%Y-%m-%d"));
    println!("Days between 2023-01-10 and 2023-01-05: {}", 
             date_utils::date_difference_days("2023-01-10", "2023-01-05")?);
    println!("Convert '2023-12-25' to DD/MM/YYYY: {}", 
             date_utils::to_dd_mm_yyyy("2023-12-25")?);
    println!("Is 2024 a leap year? {}", date_utils::is_leap_year(2024));
    Ok(())
}

fn file_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== File I/O Utils Demo ===");
    let temp_file = "/tmp/cli_utils_demo.txt";
    let content = "Hello from CLI Utils!\nThis is a demo file.";
    
    file_io_utils::write_string_to_file(temp_file, content)?;
    println!("✓ Created demo file: {}", temp_file);
    
    let read_content = file_io_utils::read_file_to_string(temp_file)?;
    println!("File content: {}", read_content.lines().next().unwrap_or(""));
    
    println!("File size: {} bytes", file_io_utils::file_size(temp_file)?);
    
    file_io_utils::delete_file(temp_file)?;
    println!("✓ Cleaned up demo file");
    
    Ok(())
}

fn interactive_mode() {
    println!("=== Interactive Mode ===");
    println!("Type 'exit' to return to main menu");
    
    loop {
        println!();
        println!("Choose operation:");
        println!("- factorial <number>");
        println!("- prime <number>");
        println!("- palindrome <text>");
        println!("- reverse <text>");
        println!("- exit");
        print!("> ");
        
        let input = read_stdin();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            continue;
        }
        
        match parts[0] {
            "exit" => break,
            "factorial" => {
                if parts.len() != 2 {
                    println!("Usage: factorial <number>");
                    continue;
                }
                if let Ok(n) = parts[1].parse::<u64>() {
                    if n <= 20 {
                        println!("Factorial of {}: {}", n, math_utils::factorial(n));
                    } else {
                        println!("Number too large (max 20)");
                    }
                } else {
                    println!("Invalid number");
                }
            },
            "prime" => {
                if parts.len() != 2 {
                    println!("Usage: prime <number>");
                    continue;
                }
                if let Ok(n) = parts[1].parse::<u64>() {
                    println!("Is {} prime? {}", n, math_utils::is_prime(n));
                } else {
                    println!("Invalid number");
                }
            },
            "palindrome" => {
                if parts.len() < 2 {
                    println!("Usage: palindrome <text>");
                    continue;
                }
                let text = parts[1..].join(" ");
                println!("Is '{}' a palindrome? {}", text, string_utils::is_palindrome(&text));
            },
            "reverse" => {
                if parts.len() < 2 {
                    println!("Usage: reverse <text>");
                    continue;
                }
                let text = parts[1..].join(" ");
                println!("Reverse of '{}': {}", text, string_utils::reverse_string(&text));
            },
            _ => println!("Unknown command. Try factorial, prime, palindrome, reverse, or exit"),
        }
    }
}