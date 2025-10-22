//! Mathematical utility functions
//! 
//! This module provides basic mathematical utility functions including:
//! - Factorial calculation
//! - Greatest Common Divisor (GCD)
//! - Prime number checking

/// Calculates the factorial of a non-negative integer
/// 
/// # Arguments
/// * `n` - A non-negative integer
/// 
/// # Returns
/// The factorial of n (n!)
/// 
/// # Panics
/// Panics if n is greater than 20 to prevent overflow
/// 
/// # Examples
/// ```
/// use cli_utils::math_utils::factorial;
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(0), 1);
/// ```
pub fn factorial(n: u64) -> u64 {
    if n > 20 {
        panic!("Factorial input too large (max 20 to prevent overflow)");
    }
    
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// Calculates the Greatest Common Divisor (GCD) of two integers using Euclid's algorithm
/// 
/// # Arguments
/// * `a` - First integer
/// * `b` - Second integer
/// 
/// # Returns
/// The GCD of a and b
/// 
/// # Examples
/// ```
/// use cli_utils::math_utils::gcd;
/// assert_eq!(gcd(48, 18), 6);
/// assert_eq!(gcd(17, 19), 1);
/// ```
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Checks if a number is prime
/// 
/// # Arguments
/// * `n` - The number to check
/// 
/// # Returns
/// `true` if the number is prime, `false` otherwise
/// 
/// # Examples
/// ```
/// use cli_utils::math_utils::is_prime;
/// assert_eq!(is_prime(17), true);
/// assert_eq!(is_prime(4), false);
/// assert_eq!(is_prime(1), false);
/// ```
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Calculates the least common multiple (LCM) of two integers
/// 
/// # Arguments
/// * `a` - First integer
/// * `b` - Second integer
/// 
/// # Returns
/// The LCM of a and b
/// 
/// # Examples
/// ```
/// use cli_utils::math_utils::lcm;
/// assert_eq!(lcm(4, 6), 12);
/// assert_eq!(lcm(7, 9), 63);
/// ```
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    #[should_panic(expected = "Factorial input too large")]
    fn test_factorial_overflow() {
        factorial(25);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(48, 18), 6);
        assert_eq!(gcd(17, 19), 1);
        assert_eq!(gcd(100, 25), 25);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(25), false);
        assert_eq!(is_prime(97), true);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(7, 9), 63);
        assert_eq!(lcm(12, 18), 36);
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(5, 0), 0);
    }
}