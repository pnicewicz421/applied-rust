use cli_utils::math_utils::*;

#[test]
fn test_factorial_integration() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(10), 3628800);
}

#[test]
fn test_gcd_integration() {
    assert_eq!(gcd(48, 18), 6);
    assert_eq!(gcd(100, 50), 50);
    assert_eq!(gcd(17, 19), 1);
}

#[test]
fn test_prime_integration() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(17), true);
    assert_eq!(is_prime(97), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(100), false);
}

#[test]
fn test_lcm_integration() {
    assert_eq!(lcm(4, 6), 12);
    assert_eq!(lcm(12, 15), 60);
    assert_eq!(lcm(7, 9), 63);
}