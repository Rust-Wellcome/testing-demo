/// Computes the [factorial] of a `u128`.
/// 
/// # Examples
/// Basic usage:
/// ```
/// # use testing_demo::factorial;
/// assert_eq!(factorial(0), Some(1));
/// assert_eq!(factorial(1), Some(1));
/// assert_eq!(factorial(3), Some(6));
/// assert_eq!(factorial(5), Some(120));
/// ```
/// 
/// `None` is returned if the result would exceed [`u128::MAX`]:
/// ```
/// # use testing_demo::factorial;
/// assert_eq!(factorial(50), None);
/// ```
/// 
/// [factorial]: https://en.wikipedia.org/wiki/Factorial
pub fn factorial(x: u128) -> Option<u128> {
    (2..=x).try_fold(1, u128::checked_mul)
}
