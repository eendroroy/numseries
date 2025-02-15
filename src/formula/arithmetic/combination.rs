use crate::formula::arithmetic::factorial;

/// Calculates the number of combinations (`nCr`) of `n` items taken `r` at a time.
///
/// # Arguments
///
/// * `n` - The total number of elements.
/// * `r` - The number of elements to choose.
///
/// # Returns
///
/// The number of ways to choose `r` elements from a set of `n` elements.
///
/// # Panics
///
/// Panics if `n` is less than `r`.
///
/// # Examples
///
/// ```
/// use numberlab::formula::arithmetic::combination;
///
/// let result = combination(5, 3);
/// assert_eq!(result, 10);
/// ```
pub fn combination(n: u128, r: u128) -> u128 {
    if n < r {
        panic!("'n' ({}) must be greater than or equal to 'r' ({})", n, r);
    }
    factorial(n) / (factorial(r) * factorial(n - r))
}
