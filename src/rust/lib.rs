///
/// # Not too terrible Prime Calculator in Rust
/// ### by denwong47
///
/// Sample output and runtime on an Apple M1 Pro:
/// Calculating all primes under 10^4:
/// - SieveMethod.ATKIN                       : 0.000039s
/// - SieveMethod.ERATOSTHENES                : 0.000017s
/// - SieveMethod.ERATOSTHENES_THREADED       : 0.000314s
///
/// Calculating all primes under 10^5:
/// - SieveMethod.ATKIN                       : 0.000286s
/// - SieveMethod.ERATOSTHENES                : 0.000138s
/// - SieveMethod.ERATOSTHENES_THREADED       : 0.000563s
///
/// Calculating all primes under 10^6:
/// - SieveMethod.ATKIN                       : 0.002254s
/// - SieveMethod.ERATOSTHENES                : 0.001361s
/// - SieveMethod.ERATOSTHENES_THREADED       : 0.001469s
///
/// Calculating all primes under 10^7:
/// - SieveMethod.ATKIN                       : 0.017580s
/// - SieveMethod.ERATOSTHENES                : 0.012332s
/// - SieveMethod.ERATOSTHENES_THREADED       : 0.027807s
///
/// Calculating all primes under 10^8:
/// - SieveMethod.ATKIN                       : 0.457447s
/// - SieveMethod.ERATOSTHENES                : 0.589925s
/// - SieveMethod.ERATOSTHENES_THREADED       : 0.217728s
///
/// Calculating all primes under 10^9:
/// - SieveMethod.ATKIN                       : 5.470739s
/// - SieveMethod.ERATOSTHENES                : 8.565013s
/// - SieveMethod.ERATOSTHENES_THREADED       : 2.614307s
///

use pyo3::prelude::*;

mod primes;
mod py_compatibility;
use py_compatibility::enums;

/// Rust library function.
///
/// Determines if a number is a prime number.
/// This is a wrapper around `is_prime_with_known_primes`, which actually does the work.
/// The difference is that `is_prime_with_known_primes` requires a list of all known primes to be passed to it, up to and including the `sqrt` of the number itself.
/// `is_prime` will enquire that list using `list_primes` first, before calling `is_prime_with_known_primes`.
#[pyfunction(num, "*", method)]
fn is_prime(
    num: u64,
    method:Option<&enums::SieveMethod>,
) -> PyResult<bool> {
    Ok(primes::is_prime(*method.unwrap_or(&enums::SieveMethod::default()), num))
}

/// Rust library function.
///
/// List all the primes within `num`.
/// Calls `prime_mask`, and apply the mask on `enumerate`.
#[pyfunction(num, "*", method)]
fn list_primes(
    num: u64,
    method:Option<&enums::SieveMethod>,
) -> PyResult<Vec<u64>> {
    Ok(primes::list_primes(*method.unwrap_or(&enums::SieveMethod::default()), num, None))
}

/// Rust library function.
///
/// Return the number of primes within `num`.
/// Calls `prime_mask`, and filter it by `true` before getting its `len`.
#[pyfunction(num, "*", method)]
fn count_primes(
    num: u64,
    method:Option<&enums::SieveMethod>,
) -> PyResult<u64> {
    Ok(primes::count_primes(*method.unwrap_or(&enums::SieveMethod::default()), num))
}

/// Rust library function.
///
/// Return the upper bound of the nth prime.
#[pyfunction]
fn upper_bound_of_nth_prime(n: u64) -> PyResult<u64> {
    Ok(primes::upper_bound_of_nth_prime(n))
}

/// Rust library function.
///
/// List the first `n` primes.
#[pyfunction(num, "*", method)]
fn list_n_primes(
    n: u64,
    method:Option<&enums::SieveMethod>,
) -> PyResult<Vec<u64>> {
    Ok(primes::list_n_primes(*method.unwrap_or(&enums::SieveMethod::default()), n))
}

/// Rust library function.
///
/// List the first `n` primes.
#[pyfunction(num, "*", method)]
fn nth_prime(
    n: u64,
    method:Option<&enums::SieveMethod>,
) -> PyResult<Option<u64>> {
    Ok(primes::nth_prime(*method.unwrap_or(&enums::SieveMethod::default()), n))
}

/// A Python module implemented in Rust.
#[pymodule]
fn lib_rust_primes(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_prime, m)?)?;
    m.add_function(wrap_pyfunction!(list_primes, m)?)?;
    m.add_function(wrap_pyfunction!(count_primes, m)?)?;
    m.add_function(wrap_pyfunction!(upper_bound_of_nth_prime, m)?)?;
    m.add_function(wrap_pyfunction!(list_n_primes, m)?)?;
    m.add_function(wrap_pyfunction!(nth_prime, m)?)?;

    m.add_class::<py_compatibility::enums::SieveMethod>()?;

    Ok(())
}
