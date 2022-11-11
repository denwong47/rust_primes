///
/// # Not too terrible Prime Calculator in Rust
/// ### by denwong47
///
/// Sample output and runtime on an Apple M1 Pro:
/// Within the first 1,000,000 numbers, there are 78,498 prime numbers.
/// Calculating all primes up to 1,000,000 had taken 0.0026658999999999997s.
/// Validating all primes up to 1,000,000 had taken 0.015052s.
///
/// Within the first 100,000,000 numbers, there are 5,761,455 prime numbers.
/// Calculating all primes up to 100,000,000 had taken 0.6158214s.
/// Validating all primes up to 100,000,000 had taken 4.697561s.
///
/// NOTE:
/// Calculations are done threaded;
/// Validation is done serially, unthreaded.


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
