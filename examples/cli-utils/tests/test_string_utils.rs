use cli_utils::string_utils::*;

#[test]
fn test_palindrome_integration() {
    assert_eq!(is_palindrome("racecar"), true);
    assert_eq!(is_palindrome("A man a plan a canal Panama"), true);
    assert_eq!(is_palindrome("hello"), false);
    assert_eq!(is_palindrome("Was it a car or a cat I saw"), true);
}

#[test]
fn test_char_count_integration() {
    assert_eq!(count_char("hello world", 'l'), 3);
    assert_eq!(count_char("programming", 'm'), 2);
    assert_eq!(count_char("rust", 'x'), 0);
}

#[test]
fn test_string_reverse_integration() {
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string("rust programming"), "gnimmargorp tsur");
    assert_eq!(reverse_string("12345"), "54321");
}

#[test]
fn test_title_case_integration() {
    assert_eq!(to_title_case("hello world"), "Hello World");
    assert_eq!(to_title_case("rust programming language"), "Rust Programming Language");
    assert_eq!(to_title_case("HELLO WORLD"), "Hello World");
}

#[test]
fn test_utility_functions_integration() {
    assert_eq!(remove_whitespace("hello world test"), "helloworldtest");
    assert_eq!(word_count("hello world programming"), 3);
    assert_eq!(is_alphabetic("HelloWorld"), true);
    assert_eq!(is_alphabetic("Hello123"), false);
}