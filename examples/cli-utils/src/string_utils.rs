//! String manipulation utility functions
//! 
//! This module provides string manipulation utilities including:
//! - Palindrome checking
//! - Character counting
//! - String reversal
//! - Case conversion utilities

/// Checks if a string is a palindrome (reads the same forwards and backwards)
/// 
/// # Arguments
/// * `s` - The string to check
/// 
/// # Returns
/// `true` if the string is a palindrome, `false` otherwise
/// 
/// # Examples
/// ```
/// use cli_utils::string_utils::is_palindrome;
/// assert_eq!(is_palindrome("racecar"), true);
/// assert_eq!(is_palindrome("hello"), false);
/// assert_eq!(is_palindrome("A man a plan a canal Panama"), true);
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let normalized: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    
    normalized == normalized.chars().rev().collect::<String>()
}

/// Counts the occurrences of a specific character in a string
/// 
/// # Arguments
/// * `s` - The string to search in
/// * `target` - The character to count
/// 
/// # Returns
/// The number of occurrences of the target character
/// 
/// # Examples
/// ```
/// use cli_utils::string_utils::count_char;
/// assert_eq!(count_char("hello world", 'l'), 3);
/// assert_eq!(count_char("rust programming", 'r'), 3);
/// ```
pub fn count_char(s: &str, target: char) -> usize {
    s.chars().filter(|&c| c == target).count()
}

/// Reverses a string
/// 
/// # Arguments
/// * `s` - The string to reverse
/// 
/// # Returns
/// A new string with characters in reverse order
/// 
/// # Examples
/// ```
/// use cli_utils::string_utils::reverse_string;
/// assert_eq!(reverse_string("hello"), "olleh");
/// assert_eq!(reverse_string("rust"), "tsur");
/// ```
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Converts a string to title case (first letter of each word capitalized)
/// 
/// # Arguments
/// * `s` - The string to convert
/// 
/// # Returns
/// A new string in title case
/// 
/// # Examples
/// ```
/// use cli_utils::string_utils::to_title_case;
/// assert_eq!(to_title_case("hello world"), "Hello World");
/// assert_eq!(to_title_case("rust programming"), "Rust Programming");
/// ```
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Removes all whitespace from a string
/// 
/// # Arguments
/// * `s` - The string to process
/// 
/// # Returns
/// A new string with all whitespace removed
/// 
/// # Examples
/// ```
/// use cli_utils::string_utils::remove_whitespace;
/// assert_eq!(remove_whitespace("hello world"), "helloworld");
/// assert_eq!(remove_whitespace("  rust  programming  "), "rustprogramming");
/// ```
pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// Counts the number of words in a string
/// 
/// # Arguments
/// * `s` - The string to count words in
/// 
/// # Returns
/// The number of words
/// 
/// # Examples
/// ```
/// use cli_utils::string_utils::word_count;
/// assert_eq!(word_count("hello world"), 2);
/// assert_eq!(word_count("  rust  programming  language  "), 3);
/// ```
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Checks if a string contains only alphabetic characters
/// 
/// # Arguments
/// * `s` - The string to check
/// 
/// # Returns
/// `true` if the string contains only alphabetic characters, `false` otherwise
/// 
/// # Examples
/// ```
/// use cli_utils::string_utils::is_alphabetic;
/// assert_eq!(is_alphabetic("hello"), true);
/// assert_eq!(is_alphabetic("hello123"), false);
/// ```
pub fn is_alphabetic(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome("racecar"), true);
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("A man a plan a canal Panama"), true);
        assert_eq!(is_palindrome("race a car"), false);
        assert_eq!(is_palindrome(""), true);
        assert_eq!(is_palindrome("a"), true);
    }

    #[test]
    fn test_count_char() {
        assert_eq!(count_char("hello world", 'l'), 3);
        assert_eq!(count_char("rust programming", 'r'), 3);
        assert_eq!(count_char("hello", 'x'), 0);
        assert_eq!(count_char("", 'a'), 0);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("rust programming"), "Rust Programming");
        assert_eq!(to_title_case("HELLO WORLD"), "Hello World");
        assert_eq!(to_title_case(""), "");
    }

    #[test]
    fn test_remove_whitespace() {
        assert_eq!(remove_whitespace("hello world"), "helloworld");
        assert_eq!(remove_whitespace("  rust  programming  "), "rustprogramming");
        assert_eq!(remove_whitespace(""), "");
        assert_eq!(remove_whitespace("   "), "");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count("  rust  programming  language  "), 3);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("   "), 0);
        assert_eq!(word_count("single"), 1);
    }

    #[test]
    fn test_is_alphabetic() {
        assert_eq!(is_alphabetic("hello"), true);
        assert_eq!(is_alphabetic("hello123"), false);
        assert_eq!(is_alphabetic(""), false);
        assert_eq!(is_alphabetic("hello world"), false); // contains space
        assert_eq!(is_alphabetic("HelloWorld"), true);
    }
}