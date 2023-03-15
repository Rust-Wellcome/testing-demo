/// Computes the [factorial] of a `u128`.
/// 
/// [factorial]: https://en.wikipedia.org/wiki/Factorial
pub fn factorial(x: u128) -> Option<u128> {
    (2..=x).try_fold(1, u128::checked_mul)
}
