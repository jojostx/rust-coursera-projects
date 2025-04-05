//! This module contains the simple maths functions for rust.

/// Returns the factorial of a number
///
/// Examples:
/// ```
/// use cli_utils::math_utils::factorial;
///
/// let num = 1;
/// let num = factorial(num);
///
/// println!("the factorial of num is {}", num);
///
/// let num = 3;
/// let num = factorial(num);
///
/// println!("the factorial of num is {}", num);
/// assert_eq!(6, num);
/// ```
pub fn factorial(num: i32) -> i32 {
    if num == 0 || num == 1 {
        return 1;
    }

    let mut result: i32 = 1;

    for i in 1..=num {
        result *= i;
    }

    result
}

/// Returns the GCD of two number
///
/// Examples:
/// ```
/// use cli_utils::math_utils::greatest_common_denominator;
///
/// let a = 16;
/// let b = 32;
/// let gcd = greatest_common_denominator(a, b);
///
/// println!("the GCD of a and b is {}", gcd);
/// assert_eq!(16, gcd);
/// ```
pub fn greatest_common_denominator(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    a
}

/// Check if a number is a prime number
///
/// Examples:
/// ```
/// use cli_utils::math_utils::is_prime_number;
///
/// let num = 16;
/// let is_prime = is_prime_number(num);
///
/// println!("is {} a prime number? answer: {}", num, is_prime);
/// assert_eq!(false, is_prime);
///
/// let num = 7;
/// let is_prime = is_prime_number(num);
///
/// println!("is {} a prime number? answer: {}", num, is_prime);
/// assert_eq!(true, is_prime);
///
/// ```
pub fn is_prime_number(a: u32) -> bool {
    if a < 2 {
        return false;
    }

    if a == 2 {
        return true;
    }

    if a % 2 == 0 {
        return false;
    }

    let limit = (a as f64).sqrt() as u32;
    for i in 2..=limit {
        if a % i == 0 {
            return false;
        }
    }

    true
}
